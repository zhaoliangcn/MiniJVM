use std::vec::Vec;
use crate::error::{RuntimeError, JvmError, Result};
use super::value::Value;
use super::method_area::Method;

#[derive(Debug, Clone)]
pub struct Frame {
    pub method: Method,
    pub pc: usize,
    pub local_variables: Vec<Value>,
    pub operand_stack: Vec<Value>,
    pub exception: Option<Value>,
    pub return_value: Option<Value>,
}

impl Frame {
    pub fn new(method: Method) -> Self {
        let max_locals = method.max_locals;
        Frame {
            method,
            pc: 0,
            local_variables: vec![Value::Null; max_locals],
            operand_stack: Vec::new(),
            exception: None,
            return_value: None,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        self.operand_stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.operand_stack.pop()
            .ok_or(JvmError::RuntimeError(RuntimeError::StackUnderflow))
    }

    pub fn peek(&self) -> Result<&Value> {
        self.operand_stack.last()
            .ok_or(JvmError::RuntimeError(RuntimeError::StackUnderflow))
    }

    pub fn dup(&mut self) -> Result<()> {
        let value = self.peek()?.clone();
        self.push(value)
    }

    pub fn dup_x1(&mut self) -> Result<()> {
        if self.operand_stack.len() < 2 {
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
        }
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.push(v2.clone())?;
        self.push(v1)?;
        self.push(v2)
    }

    pub fn dup_x2(&mut self) -> Result<()> {
        if self.operand_stack.len() < 3 {
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
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
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
        }
        let v2 = self.operand_stack[self.operand_stack.len() - 2].clone();
        let v1 = self.operand_stack[self.operand_stack.len() - 1].clone();
        self.push(v2)?;
        self.push(v1)
    }

    pub fn dup2_x1(&mut self) -> Result<()> {
        if self.operand_stack.len() < 3 {
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
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
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
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
            return Err(JvmError::RuntimeError(RuntimeError::StackUnderflow));
        }
        let len = self.operand_stack.len();
        self.operand_stack.swap(len - 1, len - 2);
        Ok(())
    }

    pub fn get_local(&self, index: usize) -> Result<&Value> {
        self.local_variables.get(index)
            .ok_or(JvmError::RuntimeError(RuntimeError::LocalVariableIndexOutOfBounds(index)))
    }

    pub fn set_local(&mut self, index: usize, value: Value) -> Result<()> {
        if index >= self.local_variables.len() {
            return Err(JvmError::RuntimeError(RuntimeError::LocalVariableIndexOutOfBounds(index)));
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

#[derive(Debug, Clone, Default)]
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
            return Err(JvmError::RuntimeError(RuntimeError::StackOverflow));
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Frame> {
        self.frames.pop()
            .ok_or(JvmError::RuntimeError(RuntimeError::StackUnderflow))
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

    pub fn get_frames(&self) -> &[Frame] {
        &self.frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::method_area::Method;

    fn dummy_method() -> Method {
        Method {
            class_name: "Test".to_string(),
            name: "test".to_string(),
            descriptor: "()V".to_string(),
            code: vec![],
            max_stack: 10,
            max_locals: 10,
            is_native: false,
            is_static: true,
            native_impl: None,
        }
    }

    #[test]
    fn test_frame_new() {
        let method = dummy_method();
        let frame = Frame::new(method);
        assert_eq!(frame.local_variables.len(), 10);
        assert!(frame.operand_stack.is_empty());
        assert_eq!(frame.pc, 0);
        assert!(frame.exception.is_none());
        assert!(frame.return_value.is_none());
    }

    #[test]
    fn test_frame_push_pop() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.push(Value::Int(1)).unwrap();
        frame.push(Value::Int(2)).unwrap();
        assert_eq!(frame.operand_stack_size(), 2);
        assert_eq!(frame.pop().unwrap(), Value::Int(2));
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
    }

    #[test]
    fn test_frame_peek() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.push(Value::Int(42)).unwrap();
        assert_eq!(*frame.peek().unwrap(), Value::Int(42));
        assert_eq!(frame.operand_stack_size(), 1);
    }

    #[test]
    fn test_frame_dup() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.push(Value::Int(1)).unwrap();
        frame.dup().unwrap();
        assert_eq!(frame.operand_stack_size(), 2);
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
    }

    #[test]
    fn test_frame_dup_x1() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.push(Value::Int(2)).unwrap();
        frame.push(Value::Int(1)).unwrap();
        frame.dup_x1().unwrap();
        // Stack should be: 1, 2, 1
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
        assert_eq!(frame.pop().unwrap(), Value::Int(2));
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
    }

    #[test]
    fn test_frame_swap() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.push(Value::Int(1)).unwrap();
        frame.push(Value::Int(2)).unwrap();
        frame.swap().unwrap();
        assert_eq!(frame.pop().unwrap(), Value::Int(1));
        assert_eq!(frame.pop().unwrap(), Value::Int(2));
    }

    #[test]
    fn test_local_variables() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        frame.set_local(0, Value::Int(42)).unwrap();
        frame.set_local(5, Value::Long(100)).unwrap();
        assert_eq!(*frame.get_local(0).unwrap(), Value::Int(42));
        assert_eq!(*frame.get_local(5).unwrap(), Value::Long(100));
    }

    #[test]
    fn test_frame_pop_underflow() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        assert!(frame.pop().is_err());
    }

    #[test]
    fn test_local_out_of_bounds() {
        let method = dummy_method();
        let mut frame = Frame::new(method);
        assert!(frame.get_local(100).is_err());
        assert!(frame.set_local(100, Value::Int(0)).is_err());
    }

    #[test]
    fn test_jvm_stack_push_pop() {
        let mut stack = JvmStack::new();
        let method = dummy_method();
        let frame = Frame::new(method);
        stack.push(frame).unwrap();
        assert_eq!(stack.depth(), 1);
        assert!(!stack.is_empty());
        let _ = stack.pop().unwrap();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_jvm_stack_overflow() {
        let mut stack = JvmStack::new();
        // Create many frames to overflow the stack
        for _ in 0..1100 {
            let method = dummy_method();
            let frame = Frame::new(method);
            if stack.push(frame).is_err() {
                return; // Expected overflow
            }
        }
        panic!("Should have overflowed");
    }

    #[test]
    fn test_jvm_stack_underflow() {
        let mut stack = JvmStack::new();
        assert!(stack.pop().is_err());
    }
}
