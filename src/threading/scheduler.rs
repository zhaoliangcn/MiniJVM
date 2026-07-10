use std::collections::{HashMap, VecDeque};
use crate::error::{ThreadingError, Result};
use super::thread::{Thread, ThreadState, ThreadPriority};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    FIFO,
    RoundRobin,
    Priority,
}

pub struct Scheduler {
    threads: HashMap<usize, Thread>,
    ready_queue: VecDeque<usize>,
    policy: SchedulingPolicy,
    current_thread: Option<usize>,
    thread_id_counter: usize,
    time_slice: usize,
}

impl Scheduler {
    pub fn new(policy: SchedulingPolicy) -> Self {
        Scheduler {
            threads: HashMap::new(),
            ready_queue: VecDeque::new(),
            policy,
            current_thread: None,
            thread_id_counter: 1,
            time_slice: 1000,
        }
    }

    pub fn create_thread(&mut self, name: String) -> Result<usize> {
        let id = self.thread_id_counter;
        self.thread_id_counter += 1;
        
        let thread = Thread::new(id, name);
        self.threads.insert(id, thread);
        
        Ok(id)
    }

    pub fn start_thread(&mut self, thread_id: usize) -> Result<()> {
        let thread = self.threads.get_mut(&thread_id)
            .ok_or(ThreadingError::ThreadCreationFailed)?;
        
        thread.state = ThreadState::Runnable;
        self.add_to_ready_queue(thread_id);
        
        Ok(())
    }

    pub fn schedule(&mut self) -> Option<usize> {
        if self.ready_queue.is_empty() {
            return None;
        }
        
        let thread_id = match self.policy {
            SchedulingPolicy::FIFO => self.ready_queue.pop_front().unwrap(),
            SchedulingPolicy::RoundRobin => {
                let id = self.ready_queue.pop_front().unwrap();
                self.ready_queue.push_back(id);
                id
            }
            SchedulingPolicy::Priority => self.select_highest_priority(),
        };
        
        if let Some(thread) = self.threads.get_mut(&thread_id) {
            thread.state = ThreadState::Running;
        }
        
        self.current_thread = Some(thread_id);
        Some(thread_id)
    }

    pub fn yield_thread(&mut self) -> Result<()> {
        if let Some(current_id) = self.current_thread {
            let thread = self.threads.get_mut(&current_id)
                .ok_or(ThreadingError::ThreadInterrupted)?;
            
            if thread.state == ThreadState::Running {
                thread.state = ThreadState::Runnable;
                self.add_to_ready_queue(current_id);
                self.current_thread = None;
            }
        }
        
        Ok(())
    }

    pub fn sleep(&mut self, thread_id: usize, _millis: u64) -> Result<()> {
        let thread = self.threads.get_mut(&thread_id)
            .ok_or(ThreadingError::ThreadInterrupted)?;
        
        thread.state = ThreadState::TimedWaiting;
        
        if Some(thread_id) == self.current_thread {
            self.current_thread = None;
        }
        
        Ok(())
    }

    pub fn join(&mut self, thread_id: usize) -> Result<()> {
        loop {
            if let Some(thread) = self.threads.get(&thread_id) {
                if thread.state == ThreadState::Terminated {
                    break;
                }
            } else {
                break;
            }
            
            self.yield_thread()?;
        }
        
        Ok(())
    }

    pub fn get_thread(&self, thread_id: usize) -> Option<&Thread> {
        self.threads.get(&thread_id)
    }

    pub fn get_thread_mut(&mut self, thread_id: usize) -> Option<&mut Thread> {
        self.threads.get_mut(&thread_id)
    }

    pub fn get_current_thread(&self) -> Option<usize> {
        self.current_thread
    }

    pub fn set_policy(&mut self, policy: SchedulingPolicy) {
        self.policy = policy;
    }

    pub fn get_policy(&self) -> SchedulingPolicy {
        self.policy
    }

    pub fn set_time_slice(&mut self, time_slice: usize) {
        self.time_slice = time_slice;
    }

    pub fn get_time_slice(&self) -> usize {
        self.time_slice
    }

    fn add_to_ready_queue(&mut self, thread_id: usize) {
        if !self.ready_queue.contains(&thread_id) {
            match self.policy {
                SchedulingPolicy::Priority => {
                    let thread = self.threads.get(&thread_id).unwrap();
                    let pos = self.ready_queue.iter()
                        .position(|&id| {
                            let other = self.threads.get(&id).unwrap();
                            thread.priority > other.priority
                        })
                        .unwrap_or(self.ready_queue.len());
                    self.ready_queue.insert(pos, thread_id);
                }
                _ => {
                    self.ready_queue.push_back(thread_id);
                }
            }
        }
    }

    fn select_highest_priority(&mut self) -> usize {
        let mut highest_id = self.ready_queue[0];
        let mut highest_priority = self.threads.get(&highest_id).unwrap().priority;
        
        for &id in &self.ready_queue {
            let priority = self.threads.get(&id).unwrap().priority;
            if priority > highest_priority {
                highest_priority = priority;
                highest_id = id;
            }
        }
        
        let idx = self.ready_queue.iter().position(|&id| id == highest_id).unwrap();
        self.ready_queue.remove(idx);
        highest_id
    }

    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler::new(SchedulingPolicy::RoundRobin)
    }
}
