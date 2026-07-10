use std::vec::Vec;
use crate::error::{RuntimeError, Result};
use super::value::Value;
use super::method_area::Method;

#[derive(Debug, Clone)]
pub struct Frame {
    pub method: Method,
    pub pc: usize,
    pub local_variables: Vec<Value>,
    pub operand_stack: Vec<Value>,
}

impl Frame {
    pub fn new(method: Method) -> Self {
        let max_locals = method.max_locals;
        Frame {
            method,
            pc: 0,
            local_variables: vec![Value::Null; max_locals],
            operand_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        self.operand_stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.operand_stack.pop()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<&Value> {
        self.operand_stack.last()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn dup(&mut self) -> Result<()> {
        let value = self.peek()?.clone();
        self.push(value)
    }

    pub fn dup_x1(&mut self) -> Result<()> {
        if self.operand_stack.len() < 2 {
            return Err(RuntimeError::StackUnderflow);
        }
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.push(v2.clone())?;
        self.push(v1)?;
        self.push(v2)
    }

    pub fn dup_x2(&mut self) -> Result<()> {
        if self.operand_stack.len() < 3 {
            return Err(RuntimeError::StackUnderflow);
        }
        let v3 = self.pop()?;
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.push(v3.clone())?;
        self.push(v1)?;
        self.push(v2)?;
        self.push(v3)
    }

    pub fn dup2(&mut self) -> Result<()> {
        if self.operand_stack.len() < 2 {
            return Err(RuntimeError::StackUnderflow);
        }
        let v2 = self.operand_stack[self.operand_stack.len() - 2].clone();
        let v1 = self.operand_stack[self.operand_stack.len() - 1].clone();
        self.push(v2)?;
        self.push(v1)
    }

    pub fn dup2_x1(&mut self) -> Result<()> {
        if self.operand_stack.len() < 3 {
            return Err(RuntimeError::StackUnderflow);
        }
        let v3 = self.pop()?;
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.push(v2.clone())?;
        self.push(v3.clone())?;
        self.push(v1)?;
        self.push(v2)?;
        self.push(v3)
    }

    pub fn dup2_x2(&mut self) -> Result<()> {
        if self.operand_stack.len() < 4 {
            return Err(RuntimeError::StackUnderflow);
        }
        let v4 = self.pop()?;
        let v3 = self.pop()?;
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.push(v3.clone())?;
        self.push(v4.clone())?;
        self.push(v1)?;
        self.push(v2)?;
        self.push(v3)?;
        self.push(v4)
    }

    pub fn swap(&mut self) -> Result<()> {
        if self.operand_stack.len() < 2 {
            return Err(RuntimeError::StackUnderflow);
        }
        let len = self.operand_stack.len();
        self.operand_stack.swap(len - 1, len - 2);
        Ok(())
    }

    pub fn get_local(&self, index: usize) -> Result<&Value> {
        self.local_variables.get(index)
            .ok_or(RuntimeError::LocalVariableIndexOutOfBounds(index))
    }

    pub fn set_local(&mut self, index: usize, value: Value) -> Result<()> {
        if index >= self.local_variables.len() {
            return Err(RuntimeError::LocalVariableIndexOutOfBounds(index));
        }
        self.local_variables[index] = value;
        Ok(())
    }

    pub fn operand_stack_size(&self) -> usize {
        self.operand_stack.len()
    }

    pub fn local_variables_size(&self) -> usize {
        self.local_variables.len()
    }
}

#[derive(Debug, Clone)]
pub struct JvmStack {
    frames: Vec<Frame>,
    max_depth: usize,
}

impl JvmStack {
    pub fn new() -> Self {
        JvmStack {
            frames: Vec::new(),
            max_depth: 1024,
        }
    }

    pub fn push(&mut self, frame: Frame) -> Result<()> {
        if self.frames.len() >= self.max_depth {
            return Err(RuntimeError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Frame> {
        self.frames.pop()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn peek(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
