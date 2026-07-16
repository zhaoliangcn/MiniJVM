use std::collections::{HashSet, VecDeque, HashMap};
use crate::error::{GcError, JvmError, Result};
use crate::runtime::{Heap, HeapObject, JvmStack, Frame, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcAlgorithm {
    MarkSweep,
    Copying,
    MarkCompact,
    Generational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcState {
    Idle,
    Marking,
    Sweeping,
    Copying,
    Paused,
}

pub struct GcCollector {
    algorithm: GcAlgorithm,
    state: GcState,
    threshold: usize,
    allocated_since_last_gc: usize,
}

impl GcCollector {
    pub fn new(algorithm: GcAlgorithm) -> Self {
        GcCollector {
            algorithm,
            state: GcState::Idle,
            threshold: 1024,
            allocated_since_last_gc: 0,
        }
    }

    pub fn allocate(&mut self, heap: &mut Heap, object: HeapObject) -> Result<usize> {
        let id = heap.allocate(object)?;
        self.allocated_since_last_gc += 1;
        
        if self.allocated_since_last_gc >= self.threshold {
            self.collect(heap)?;
        }
        
        Ok(id)
    }

    pub fn collect(&mut self, heap: &mut Heap) -> Result<()> {
        if self.state != GcState::Idle {
            return Err(JvmError::GcError(GcError::Interrupted));
        }
        
        self.state = GcState::Marking;
        self.allocated_since_last_gc = 0;
        
        match self.algorithm {
            GcAlgorithm::MarkSweep => self.mark_sweep(heap)?,
            GcAlgorithm::Copying => self.copying(heap)?,
            GcAlgorithm::MarkCompact => self.mark_compact(heap)?,
            GcAlgorithm::Generational => self.generational(heap)?,
        }
        
        self.state = GcState::Idle;
        Ok(())
    }

    pub fn collect_with_stack(&mut self, heap: &mut Heap, stack: &JvmStack) -> Result<()> {
        if self.state != GcState::Idle {
            return Err(JvmError::GcError(GcError::Interrupted));
        }
        
        self.state = GcState::Marking;
        self.allocated_since_last_gc = 0;
        
        match self.algorithm {
            GcAlgorithm::MarkSweep => self.mark_sweep_with_stack(heap, stack)?,
            GcAlgorithm::Copying => self.copying_with_stack(heap, stack)?,
            GcAlgorithm::MarkCompact => self.mark_compact_with_stack(heap, stack)?,
            GcAlgorithm::Generational => self.generational_with_stack(heap, stack)?,
        }
        
        self.state = GcState::Idle;
        Ok(())
    }

    fn mark_sweep(&mut self, heap: &mut Heap) -> Result<()> {
        let mut reachable = HashSet::new();
        self.mark(heap, Vec::new(), &mut reachable)?;
        self.sweep(heap, &reachable)?;
        Ok(())
    }

    fn mark_sweep_with_stack(&mut self, heap: &mut Heap, stack: &JvmStack) -> Result<()> {
        let mut reachable = HashSet::new();
        let roots = self.find_roots_from_stack(stack);
        self.mark(heap, roots, &mut reachable)?;
        self.sweep(heap, &reachable)?;
        Ok(())
    }

    fn marking(&self, heap: &Heap, stack: &JvmStack) -> Result<HashSet<usize>> {
        let mut reachable = HashSet::new();
        let roots = self.find_roots_from_stack(stack);
        self.mark(heap, roots, &mut reachable)?;
        Ok(reachable)
    }

    fn sweep(&mut self, heap: &mut Heap, reachable: &HashSet<usize>) -> Result<()> {
        for i in 1..heap.len() {
            if let Some(_) = heap.get(i) {
                if !reachable.contains(&i) {
                    heap.deallocate(i)?;
                }
            }
        }
        Ok(())
    }

    fn copying(&mut self, heap: &mut Heap) -> Result<()> {
        let mut reachable = HashSet::new();
        self.mark(heap, Vec::new(), &mut reachable)?;
        
        let mut new_heap = Heap::new();
        let mut old_to_new = HashMap::new();
        
        for &id in &reachable {
            if let Some(obj) = heap.get(id) {
                let new_id = new_heap.allocate(obj.clone())?;
                old_to_new.insert(id, new_id);
            }
        }
        
        *heap = new_heap;
        Ok(())
    }

    fn copying_with_stack(&mut self, heap: &mut Heap, stack: &JvmStack) -> Result<()> {
        let mut reachable = HashSet::new();
        let roots = self.find_roots_from_stack(stack);
        self.mark(heap, roots, &mut reachable)?;
        
        let mut new_heap = Heap::new();
        let mut old_to_new = HashMap::new();
        
        for &id in &reachable {
            if let Some(obj) = heap.get(id) {
                let new_id = new_heap.allocate(obj.clone())?;
                old_to_new.insert(id, new_id);
            }
        }
        
        *heap = new_heap;
        Ok(())
    }

    fn mark_compact(&mut self, heap: &mut Heap) -> Result<()> {
        let mut reachable = HashSet::new();
        self.mark(heap, Vec::new(), &mut reachable)?;
        
        let mut new_heap = Heap::new();
        
        for i in 1..heap.len() {
            if let Some(obj) = heap.get(i) {
                if reachable.contains(&i) {
                    new_heap.allocate(obj.clone())?;
                }
            }
        }
        
        *heap = new_heap;
        Ok(())
    }

    fn mark_compact_with_stack(&mut self, heap: &mut Heap, stack: &JvmStack) -> Result<()> {
        let mut reachable = HashSet::new();
        let roots = self.find_roots_from_stack(stack);
        self.mark(heap, roots, &mut reachable)?;
        
        let mut new_heap = Heap::new();
        
        for i in 1..heap.len() {
            if let Some(obj) = heap.get(i) {
                if reachable.contains(&i) {
                    new_heap.allocate(obj.clone())?;
                }
            }
        }
        
        *heap = new_heap;
        Ok(())
    }

    fn generational(&mut self, heap: &mut Heap) -> Result<()> {
        // Fallback to full mark-sweep when no stack is available
        self.mark_sweep(heap)
    }

    fn generational_with_stack(&mut self, heap: &mut Heap, stack: &JvmStack) -> Result<()> {
        let roots = self.find_roots_from_stack(stack);
        
        // Phase 1: Young collection — collect only young objects (generation 0)
        let mut young_reachable = HashSet::new();
        let mut young_roots = roots.iter()
            .filter(|&&id| heap.get(id).map(|o| o.generation == 0).unwrap_or(false))
            .copied()
            .collect::<Vec<_>>();
        // Also include roots that point to old objects, since they might reference young ones
        let mut all_roots = roots.clone();
        
        // Mark young reachable objects starting from all roots
        let mut queue = VecDeque::from(all_roots.clone());
        let mut visited = HashSet::new();
        
        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id);
            
            if let Some(obj) = heap.get(id) {
                if obj.generation == 0 {
                    young_reachable.insert(id);
                }
                // Follow references from all objects
                for val in obj.fields.values() {
                    if let Value::ObjectRef(ref_id) = val {
                        if *ref_id != 0 && !visited.contains(ref_id) {
                            queue.push_back(*ref_id);
                        }
                    } else if let Value::ArrayRef(ref_id) = val {
                        if *ref_id != 0 && !visited.contains(ref_id) {
                            queue.push_back(*ref_id);
                        }
                    }
                }
                if let Some(elements) = &obj.array_elements {
                    for element in elements {
                        if let Value::ObjectRef(ref_id) = element {
                            if *ref_id != 0 && !visited.contains(ref_id) {
                                queue.push_back(*ref_id);
                            }
                        } else if let Value::ArrayRef(ref_id) = element {
                            if *ref_id != 0 && !visited.contains(ref_id) {
                                queue.push_back(*ref_id);
                            }
                        }
                    }
                }
            }
        }
        
        // Sweep young generation: remove unreachable young objects,
        // promote surviving young objects (increment age, move to old gen if age >= threshold)
        let promotion_age_threshold = 2; // Promote after surviving 2 young GCs
        let mut promoted_count = 0;
        let mut swept_young = 0;
        
        for i in 1..heap.len() {
            if let Some(obj) = heap.get(i) {
                if obj.generation == 0 {
                    if young_reachable.contains(&i) {
                        // Surviving young object: increment age, promote if old enough
                        if let Some(obj) = heap.get_mut(i) {
                            obj.age += 1;
                            if obj.age >= promotion_age_threshold {
                                obj.generation = 1; // Promote to old generation
                                promoted_count += 1;
                            }
                        }
                    } else {
                        // Unreachable young object: sweep
                        heap.deallocate(i)?;
                        swept_young += 1;
                    }
                }
            }
        }
        
        // Phase 2: If promotion is still failing (high allocation), run full GC
        // Simple heuristic: if young GC didn't free enough, run full mark-sweep
        let young_total = heap.len();
        if young_total > 0 && swept_young == 0 && promoted_count == 0 {
            // Nothing was freed or promoted — run full GC
            self.mark_sweep_with_stack(heap, stack)?;
        }
        
        Ok(())
    }

    fn find_roots_from_stack(&self, stack: &JvmStack) -> Vec<usize> {
        let mut roots = Vec::new();
        
        let frames = stack.get_frames();
        for frame in frames {
            for val in &frame.local_variables {
                if let Value::ObjectRef(id) = val {
                    if *id != 0 {
                        roots.push(*id);
                    }
                } else if let Value::ArrayRef(id) = val {
                    if *id != 0 {
                        roots.push(*id);
                    }
                }
            }
            
            for val in &frame.operand_stack {
                if let Value::ObjectRef(id) = val {
                    if *id != 0 {
                        roots.push(*id);
                    }
                } else if let Value::ArrayRef(id) = val {
                    if *id != 0 {
                        roots.push(*id);
                    }
                }
            }
        }
        
        roots
    }

    fn mark(&self, heap: &Heap, mut roots: Vec<usize>, reachable: &mut HashSet<usize>) -> Result<()> {
        let mut queue = VecDeque::from(roots);
        
        while let Some(id) = queue.pop_front() {
            if reachable.contains(&id) {
                continue;
            }
            
            reachable.insert(id);
            
            if let Some(obj) = heap.get(id) {
                for val in obj.fields.values() {
                    if let Value::ObjectRef(ref_id) = val {
                        if *ref_id != 0 && !reachable.contains(ref_id) {
                            queue.push_back(*ref_id);
                        }
                    } else if let Value::ArrayRef(ref_id) = val {
                        if *ref_id != 0 && !reachable.contains(ref_id) {
                            queue.push_back(*ref_id);
                        }
                    }
                }
                
                if let Some(elements) = &obj.array_elements {
                    for element in elements {
                        if let Value::ObjectRef(ref_id) = element {
                            if *ref_id != 0 && !reachable.contains(ref_id) {
                                queue.push_back(*ref_id);
                            }
                        } else if let Value::ArrayRef(ref_id) = element {
                            if *ref_id != 0 && !reachable.contains(ref_id) {
                                queue.push_back(*ref_id);
                            }
                        }
                    }
                }
                
                if let Some(_) = &obj.string_value {
                }
            }
        }
        
        Ok(())
    }

    pub fn get_state(&self) -> GcState {
        self.state
    }

    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }

    pub fn get_threshold(&self) -> usize {
        self.threshold
    }

    pub fn increment_allocated(&mut self) {
        self.allocated_since_last_gc += 1;
    }

    pub fn should_collect(&self) -> bool {
        self.allocated_since_last_gc >= self.threshold
    }
}

impl Default for GcCollector {
    fn default() -> Self {
        GcCollector::new(GcAlgorithm::MarkSweep)
    }
}
