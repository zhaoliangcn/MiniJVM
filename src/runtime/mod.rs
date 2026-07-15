pub mod value;
pub mod heap;
pub mod stack;
pub mod method_area;

pub use value::Value;
pub use heap::{Heap, HeapObject};
pub use stack::{JvmStack, Frame};
pub use method_area::MethodArea;

use std::collections::HashMap;
use crate::gc::collector::GcCollector;
use crate::threading::scheduler::{Scheduler, SchedulingPolicy};

pub struct JVM {
    pub method_area: MethodArea,
    pub heap: Heap,
    pub stack: JvmStack,
    pub scheduler: Scheduler,
    pub current_thread_id: usize,
    /// Maps internal thread ID -> Java Thread object heap ref
    pub thread_objects: HashMap<usize, usize>,
    /// Garbage collector
    pub gc: GcCollector,
}

impl JVM {
    pub fn new() -> Self {
        let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin);
        // Create the main thread (id=1)
        let main_thread_id = scheduler.create_thread("main".to_string())
            .unwrap_or(1);
        JVM {
            method_area: MethodArea::new(),
            heap: Heap::new(),
            stack: JvmStack::new(),
            scheduler,
            current_thread_id: main_thread_id,
            thread_objects: HashMap::new(),
            gc: GcCollector::default(),
        }
    }

    /// Allocate an object on the heap, triggering GC if the allocation threshold is exceeded.
    /// The current thread's stack is scanned for GC roots.
    pub fn allocate(&mut self, object: HeapObject) -> Result<usize, crate::error::JvmError> {
        let id = self.heap.allocate(object)?;
        self.gc.increment_allocated();
        if self.gc.should_collect() {
            // Save the stack temporarily for GC root scanning
            let stack = std::mem::replace(&mut self.stack, JvmStack::new());
            self.gc.collect_with_stack(&mut self.heap, &stack)?;
            self.stack = stack;
        }
        Ok(id)
    }

    /// Save the current JVM stack into the current thread's saved stack.
    pub fn save_current_stack(&mut self) {
        let stack = std::mem::replace(&mut self.stack, JvmStack::new());
        self.scheduler.save_stack(self.current_thread_id, stack);
    }

    /// Load a thread's stack into the JVM's current stack.
    pub fn load_thread_stack(&mut self, thread_id: usize) {
        let stack = self.scheduler.take_stack(thread_id);
        self.stack = stack;
        self.current_thread_id = thread_id;
    }

    /// Switch execution to a different thread, saving the current stack first.
    pub fn switch_to_thread(&mut self, thread_id: usize) {
        self.save_current_stack();
        self.load_thread_stack(thread_id);
    }
}
