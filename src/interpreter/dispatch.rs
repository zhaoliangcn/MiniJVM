use crate::error::{InterpreterError, RuntimeError, JvmError, Result};
use crate::runtime::{JVM, Frame, Value};
use crate::threading::thread::ThreadState;
use super::instruction_set::InstructionSet;

pub struct Interpreter {
    instruction_set: InstructionSet,
    max_instructions_per_tick: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            instruction_set: InstructionSet::new(),
            max_instructions_per_tick: 10000,
        }
    }

    /// Run the interpreter for a single thread until its stack is empty (thread terminates).
    pub fn run(&self, jvm: &mut JVM, thread_id: usize) -> Result<()> {
        jvm.load_thread_stack(thread_id);
        
        self.run_inner(jvm, usize::MAX)?;
        
        // Mark thread as terminated
        jvm.scheduler.set_thread_terminated(thread_id);
        jvm.save_current_stack();
        
        Ok(())
    }

    /// Run the interpreter for a single thread for up to `max_instructions` instructions,
    /// then save the stack and return. Used for multi-threaded scheduling.
    pub fn run_tick(&self, jvm: &mut JVM, thread_id: usize) -> Result<()> {
        jvm.load_thread_stack(thread_id);
        
        self.run_inner(jvm, self.max_instructions_per_tick)?;
        
        jvm.save_current_stack();
        Ok(())
    }

    /// Main scheduling loop — runs all threads cooperatively.
    pub fn run_multi(&self, jvm: &mut JVM) -> Result<()> {
        loop {
            // Schedule the next thread
            let thread_id = match jvm.scheduler.schedule() {
                Some(id) => id,
                None => break, // No runnable threads
            };
            
            // Run this thread for a tick
            self.run_tick(jvm, thread_id)?;
            
            // Check if the thread is still runnable; if terminated, remove from ready queue
            if jvm.scheduler.is_thread_terminated(thread_id) {
                // Thread is done, try next one
                continue;
            }
            
            // Re-add to ready queue for next scheduling cycle
            let thread = jvm.scheduler.get_thread(thread_id);
            let is_runnable = thread.map(|t| t.get_state() == ThreadState::Runnable).unwrap_or(false);
            if !is_runnable {
                // Thread is blocked/waiting, don't re-add
                continue;
            }
            
            // Yield the thread (re-add to ready queue)
            let _ = jvm.scheduler.yield_thread();
        }
        
        Ok(())
    }

    /// Inner interpreter loop, runs for at most `max_instructions` instructions.
    fn run_inner(&self, jvm: &mut JVM, max_instructions: usize) -> Result<()> {
        let mut instruction_count = 0usize;
        
        loop {
            if jvm.stack.is_empty() {
                break;
            }
            
            if instruction_count >= max_instructions {
                break;
            }
            
            let mut frame = jvm.stack.pop()?;
            
            if frame.method.is_native {
                if let Some(native_impl) = frame.method.native_impl.clone() {
                    match native_impl(&mut frame, jvm) {
                        Ok(_) => {},
                        Err(e) => {
                            if let Some(exception) = self.create_exception(jvm, &e) {
                                frame.exception = Some(exception);
                            }
                        }
                    }
                }
                
                if let Some(exception) = frame.exception.take() {
                    if !jvm.stack.is_empty() {
                        let mut caller_frame = jvm.stack.pop()?;
                        caller_frame.exception = Some(exception);
                        jvm.stack.push(caller_frame)?;
                    }
                } else if !jvm.stack.is_empty() {
                    let mut caller_frame = jvm.stack.pop()?;
                    if let Some(ret_val) = frame.operand_stack.pop() {
                        caller_frame.push(ret_val)?;
                    }
                    jvm.stack.push(caller_frame)?;
                }
                
                instruction_count += 1;
                continue;
            }

            if frame.pc >= frame.method.code.len() {
                if let Some(return_value) = frame.return_value.take() {
                    if !jvm.stack.is_empty() {
                        let mut caller_frame = jvm.stack.pop()?;
                        caller_frame.push(return_value)?;
                        jvm.stack.push(caller_frame)?;
                    }
                }
                continue;
            }

            if let Some(exception) = frame.exception.take() {
                let handled = self.handle_exception(&mut frame, jvm, exception)?;
                if handled {
                    jvm.stack.push(frame)?;
                } else if !jvm.stack.is_empty() {
                    if let Some(ex) = frame.exception.take() {
                        let mut caller_frame = jvm.stack.pop()?;
                        caller_frame.exception = Some(ex);
                        jvm.stack.push(caller_frame)?;
                    }
                }
                instruction_count += 1;
                continue;
            }

            let opcode = frame.method.code[frame.pc];
            let is_call = matches!(opcode, 0xb6 | 0xb7 | 0xb8 | 0xb9 | 0xba);
            let handler = self.instruction_set.get_handler(opcode)
                .ok_or(InterpreterError::UnknownOpcode(opcode))?;

            let result = handler(&mut frame, jvm);
            
            match result {
                Ok(pc_increment) => {
                    if is_call {
                        // Call handlers (invokevirtual, invokespecial, etc.) already push
                        // the caller frame back. Only push it here if the handler didn't
                        // (e.g. invokedynamic which processes inline).
                        if jvm.stack.is_empty() {
                            jvm.stack.push(frame)?;
                        }
                        // The handler already advanced the PC and pushed the caller frame.
                        // Do not apply pc_increment here — the handler did it.
                        continue;
                    }
                    
                    if let Some(exception) = frame.exception.take() {
                        let handled = self.handle_exception(&mut frame, jvm, exception)?;
                        if handled {
                            jvm.stack.push(frame)?;
                        } else if !jvm.stack.is_empty() {
                            if let Some(ex) = frame.exception.take() {
                                let mut caller_frame = jvm.stack.pop()?;
                                caller_frame.exception = Some(ex);
                                jvm.stack.push(caller_frame)?;
                            }
                        }
                    } else {
                        if frame.pc < frame.method.code.len() {
                            frame.pc += pc_increment;
                        }
                        jvm.stack.push(frame)?;
                    }
                }
                Err(e) => {
                    if let Some(exception) = self.create_exception(jvm, &e) {
                        let handled = self.handle_exception(&mut frame, jvm, exception)?;
                        if handled {
                            jvm.stack.push(frame)?;
                        } else if !jvm.stack.is_empty() {
                            if let Some(ex) = frame.exception.take() {
                                let mut caller_frame = jvm.stack.pop()?;
                                caller_frame.exception = Some(ex);
                                jvm.stack.push(caller_frame)?;
                            }
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
            
            instruction_count += 1;
        }

        Ok(())
    }

    fn handle_exception(&self, frame: &mut Frame, jvm: &mut JVM, exception: Value) -> Result<bool> {
        let class = jvm.method_area.get_class(&frame.method.class_name);
        if let Some(class) = class {
            if let Some(code_attr) = class.get_code_attribute(&frame.method.name, &frame.method.descriptor) {
                for entry in &code_attr.exception_table {
                    if frame.pc >= entry.start_pc && frame.pc < entry.end_pc {
                        if entry.catch_type == 0 {
                            frame.pc = entry.handler_pc;
                            frame.operand_stack.clear();
                            frame.push(exception)?;
                            return Ok(true);
                        }
                        
                        let catch_class_name = class.class_file.constant_pool.get_class_name(entry.catch_type);
                        if let Some(catch_class_name) = catch_class_name {
                            let exception_obj = jvm.heap.get(exception.as_ref());
                            if let Some(exception_obj) = exception_obj {
                                let target_class = catch_class_name.replace('/', ".");
                                if self.is_assignable_from(&exception_obj.class_name, &target_class) {
                                    frame.pc = entry.handler_pc;
                                    frame.operand_stack.clear();
                                    frame.push(exception)?;
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        frame.exception = Some(exception);
        Ok(false)
    }

    fn is_assignable_from(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        
        let hierarchy = [
            ("java.lang.ArrayIndexOutOfBoundsException", "java.lang.RuntimeException"),
            ("java.lang.ArrayIndexOutOfBoundsException", "java.lang.Exception"),
            ("java.lang.NullPointerException", "java.lang.RuntimeException"),
            ("java.lang.NullPointerException", "java.lang.Exception"),
            ("java.lang.ClassCastException", "java.lang.RuntimeException"),
            ("java.lang.ClassCastException", "java.lang.Exception"),
            ("java.lang.IllegalArgumentException", "java.lang.RuntimeException"),
            ("java.lang.IllegalArgumentException", "java.lang.Exception"),
            ("java.lang.NegativeArraySizeException", "java.lang.RuntimeException"),
            ("java.lang.NegativeArraySizeException", "java.lang.Exception"),
            ("java.lang.RuntimeException", "java.lang.Exception"),
            ("java.lang.Exception", "java.lang.Throwable"),
            ("java.lang.Error", "java.lang.Throwable"),
        ];
        
        for (sub, super_) in hierarchy.iter() {
            if from == *sub && to == *super_ {
                return true;
            }
        }
        
        false
    }

    fn create_exception(&self, jvm: &mut JVM, error: &JvmError) -> Option<Value> {
        let class_name = match error {
            JvmError::RuntimeError(RuntimeError::NullPointerException) => "java.lang.NullPointerException",
            JvmError::RuntimeError(RuntimeError::ArrayIndexOutOfBounds(_)) => "java.lang.ArrayIndexOutOfBoundsException",
            JvmError::RuntimeError(RuntimeError::ClassCastException) => "java.lang.ClassCastException",
            JvmError::RuntimeError(RuntimeError::NegativeArraySize) => "java.lang.NegativeArraySizeException",
            _ => return None,
        };
        
        let target_class = jvm.method_area.get_class(class_name);
        if target_class.is_none() {
            return None;
        }
        
        let target_class = target_class.unwrap();
        let mut obj = crate::runtime::heap::HeapObject::new(class_name.to_string());
        for field_key in &target_class.instance_fields {
            obj.fields.insert(field_key.clone(), Value::Null);
        }
        
        let ref_id = jvm.allocate(obj).ok()?;
        Some(Value::ObjectRef(ref_id))
    }
}
