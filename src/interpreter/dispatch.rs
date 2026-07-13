use crate::error::{InterpreterError, RuntimeError, JvmError, Result};
use crate::runtime::{JVM, Frame, Value};
use super::instruction_set::InstructionSet;

pub struct Interpreter {
    instruction_set: InstructionSet,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            instruction_set: InstructionSet::new(),
        }
    }

    pub fn run(&self, jvm: &mut JVM) -> Result<()> {
        loop {
            if jvm.stack.is_empty() {
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
                continue;
            }

            let opcode = frame.method.code[frame.pc];
            let is_call = matches!(opcode, 0xb6 | 0xb7 | 0xb8);
            let handler = self.instruction_set.get_handler(opcode)
                .ok_or(InterpreterError::UnknownOpcode(opcode))?;

            let result = handler(&mut frame, jvm);
            
            match result {
                Ok(pc_increment) => {
                    if is_call {
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
        
        let ref_id = jvm.heap.allocate(obj).ok()?;
        Some(Value::ObjectRef(ref_id))
    }
}
