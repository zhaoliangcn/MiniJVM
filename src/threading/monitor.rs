use std::collections::VecDeque;
use crate::error::{ThreadingError, JvmError, Result};
use super::thread::Thread;

pub struct Monitor {
    owner: Option<usize>,
    entry_count: usize,
    waiters: VecDeque<usize>,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            owner: None,
            entry_count: 0,
            waiters: VecDeque::new(),
        }
    }

    pub fn enter(&mut self, thread_id: usize) -> Result<()> {
        if self.owner == Some(thread_id) {
            self.entry_count += 1;
            return Ok(());
        }
        
        if self.owner.is_some() {
            self.waiters.push_back(thread_id);
            return Err(JvmError::ThreadingError(ThreadingError::IllegalMonitorState));
        }
        
        self.owner = Some(thread_id);
        self.entry_count = 1;
        
        Ok(())
    }

    pub fn exit(&mut self, thread_id: usize) -> Result<()> {
        if self.owner != Some(thread_id) {
            return Err(JvmError::ThreadingError(ThreadingError::IllegalMonitorState));
        }
        
        self.entry_count -= 1;
        
        if self.entry_count == 0 {
            self.owner = None;
            
            if let Some(next_thread) = self.waiters.pop_front() {
                self.owner = Some(next_thread);
                self.entry_count = 1;
            }
        }
        
        Ok(())
    }

    pub fn wait(&mut self, thread_id: usize) -> Result<()> {
        if self.owner != Some(thread_id) {
            return Err(JvmError::ThreadingError(ThreadingError::IllegalMonitorState));
        }
        
        self.entry_count -= 1;
        
        if self.entry_count == 0 {
            self.owner = None;
            if let Some(next_thread) = self.waiters.pop_front() {
                self.owner = Some(next_thread);
                self.entry_count = 1;
            }
        }
        
        self.waiters.push_back(thread_id);
        
        Ok(())
    }

    pub fn notify(&mut self) -> Result<()> {
        if self.owner.is_none() {
            return Err(JvmError::ThreadingError(ThreadingError::IllegalMonitorState));
        }
        
        if let Some(thread_id) = self.waiters.pop_front() {
            self.waiters.push_back(thread_id);
        }
        
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<()> {
        if self.owner.is_none() {
            return Err(JvmError::ThreadingError(ThreadingError::IllegalMonitorState));
        }
        
        self.waiters.clear();
        
        Ok(())
    }

    pub fn get_owner(&self) -> Option<usize> {
        self.owner
    }

    pub fn get_entry_count(&self) -> usize {
        self.entry_count
    }

    pub fn get_waiters_count(&self) -> usize {
        self.waiters.len()
    }

    pub fn is_owned(&self) -> bool {
        self.owner.is_some()
    }
}
