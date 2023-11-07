//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
    },
};
use crate::timer::get_time_us;
use crate::mm::translated_to_physical_address;
use crate::task::munmap;
use crate::task::mmap;
use crate::task::current_user_token;
use crate::task::get_current_status;
use crate::task::get_syscall_times;
use crate::task::get_current_start_time;


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
    let mt = get_time_us();
    let ts = translated_to_physical_address(current_user_token(),_ts as *const u8 ) as *mut TimeVal;
    unsafe{
        *ts = TimeVal{
            sec: mt / 1_000_000,
            usec: mt % 1_000_000,
        };

    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let _ti =  translated_to_physical_address(current_user_token(),ti as *const u8 ) as *mut TaskInfo;
    unsafe{
    *_ti = TaskInfo{
        status:get_current_status(),
        syscall_times:get_syscall_times(),
        time : (get_time_us() - get_current_start_time())/1000
    };
}
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _len == 0 {
        return 0;
    }
    if  ((_port & (!0x7)) != 0) || ((_port & 0x7) == 0) || ((_start % 4096) != 0) {
        return -1;
    }
    mmap(_start,_len,_port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if _len == 0 {
        return 0;
    }
    if _start % 4096 != 0 {
        return -1;
    }
   munmap(_start,_len)
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
