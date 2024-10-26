//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_buffer,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_info,
        mmap_program, suspend_current_and_run_next, TaskStatus,
        munmap_program
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
/// save time
pub struct TimeVal {
    /// sec
    pub sec: usize,
    /// usec
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    unsafe {
        let mut buffers =
            translated_byte_buffer(current_user_token(), _ts as *const u8, size_of::<TimeVal>());
        if buffers.len() == 1 && buffers[0].len() == size_of::<TimeVal>() {
            let buf = &mut buffers[0];
            let ts = (*buf).as_mut_ptr() as *mut TimeVal;
            let us = get_time_us();

            *ts = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
            0
        } else {
            error!("sys get time error");
            -1
        }
    }
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info ");
    unsafe {
        let mut buffers = translated_byte_buffer(
            current_user_token(),
            _ti as *const u8,
            size_of::<TaskInfo>(),
        );
        if buffers.len() == 1 && buffers[0].len() == size_of::<TaskInfo>() {
            let buf = &mut buffers[0];
            let ts = (*buf).as_mut_ptr() as *mut TaskInfo;

            *ts = get_current_task_info();
            0
        } else {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
/// todo
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");

    if mmap_program(_start, _len, _port).is_some() {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
/// todo
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if munmap_program(_start, _len).is_some() {
        0
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
