use crate::error::{InterpreterError, Result};
use crate::runtime::{JVM, Frame};
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
                    native_impl(&mut frame, jvm)?;
                }
                continue;
            }

            if frame.pc >= frame.method.code.len() {
                continue;
            }

            let opcode = frame.method.code[frame.pc];
            let handler = self.instruction_set.get_handler(opcode)
                .ok_or(InterpreterError::UnknownOpcode(opcode))?;

            let pc_increment = handler(&mut frame, jvm)?;
            
            if frame.pc < frame.method.code.len() {
                frame.pc += pc_increment;
            }
            
            jvm.stack.push(frame)?;
        }

        Ok(())
    }

    pub fn execute_frame(&self, frame: &mut Frame, jvm: &mut JVM) -> Result<bool> {
        if frame.pc >= frame.method.code.len() {
            return Ok(true);
        }

        let opcode = frame.method.code[frame.pc];
        let handler = self.instruction_set.get_handler(opcode)
            .ok_or(InterpreterError::UnknownOpcode(opcode))?;

        let pc_increment = handler(frame, jvm)?;
        
        if frame.pc < frame.method.code.len() {
            frame.pc += pc_increment;
        }

        Ok(frame.pc >= frame.method.code.len())
    }
}
