use std::sync::{Arc, Mutex, Condvar};
use std::thread as std_thread;
use crate::error::{ThreadingError, Result};
use crate::runtime::{JvmStack, Frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Runnable,
    Running,
    Blocked,
    Waiting,
    TimedWaiting,
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadPriority {
    Min = 1,
    Low = 2,
    Normal = 5,
    High = 8,
    Max = 10,
}

pub struct Thread {
    id: usize,
    name: String,
    state: ThreadState,
    priority: ThreadPriority,
    stack: JvmStack,
    native_thread: Option<std_thread::JoinHandle<()>>,
    monitor: Option<usize>,
}

impl Thread {
    pub fn new(id: usize, name: String) -> Self {
        Thread {
            id,
            name,
            state: ThreadState::New,
            priority: ThreadPriority::Normal,
            stack: JvmStack::new(),
            native_thread: None,
            monitor: None,
        }
    }

    pub fn start(&mut self, entry_frame: Frame) -> Result<()> {
        if self.state != ThreadState::New {
            return Err(ThreadingError::ThreadCreationFailed);
        }
        
        self.stack.push(entry_frame)?;
        self.state = ThreadState::Runnable;
        
        Ok(())
    }

    pub fn run(&mut self) {
        self.state = ThreadState::Running;
    }

    pub fn stop(&mut self) {
        self.state = ThreadState::Terminated;
    }

    pub fn suspend(&mut self) {
        if self.state == ThreadState::Running {
            self.state = ThreadState::Blocked;
        }
    }

    pub fn resume(&mut self) {
        if self.state == ThreadState::Blocked {
            self.state = ThreadState::Runnable;
        }
    }

    pub fn wait(&mut self) {
        if self.state == ThreadState::Running {
            self.state = ThreadState::Waiting;
        }
    }

    pub fn notify(&mut self) {
        if self.state == ThreadState::Waiting || self.state == ThreadState::TimedWaiting {
            self.state = ThreadState::Runnable;
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_state(&self) -> ThreadState {
        self.state
    }

    pub fn get_priority(&self) -> ThreadPriority {
        self.priority
    }

    pub fn set_priority(&mut self, priority: ThreadPriority) {
        self.priority = priority;
    }

    pub fn get_stack(&self) -> &JvmStack {
        &self.stack
    }

    pub fn get_stack_mut(&mut self) -> &mut JvmStack {
        &mut self.stack
    }

    pub fn set_monitor(&mut self, monitor: Option<usize>) {
        self.monitor = monitor;
    }

    pub fn get_monitor(&self) -> Option<usize> {
        self.monitor
    }
}
