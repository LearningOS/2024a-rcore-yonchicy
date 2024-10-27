//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::{
    cmp,
    ops::{Add, AddAssign},
};
use lazy_static::*;

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        let p = task.inner_exclusive_access().priority;
        task.inner_exclusive_access().stride += Stride(BIG_STRIDE / p as u8);
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut i = 0;
        let mut min_t = self.ready_queue.front().unwrap().clone();
        for (idx, task) in self.ready_queue.iter().enumerate() {
            if idx == i {
                continue;
            }
            if task.inner_exclusive_access().stride < min_t.inner_exclusive_access().stride {
                i = idx;
                min_t = task.clone();
            }
        }
        self.ready_queue.remove(i)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

const BIG_STRIDE: u8 = 255;
/// stride struct for Stride scheduler
pub struct Stride(u8);
impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let max = cmp::max(self.0, other.0);
        let min = cmp::min(self.0, other.0);
        if max == min {
            Some(cmp::Ordering::Equal)
        } else if max - min <= BIG_STRIDE / 2 {
            return self.0.partial_cmp(&other.0);
        } else {
            return other.0.partial_cmp(&self.0);
        }
    }
}
impl AddAssign for Stride {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}
impl Add for Stride {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stride(self.0.wrapping_add(rhs.0))
    }
}
impl Default for Stride {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
