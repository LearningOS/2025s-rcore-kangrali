//! Process management syscalls
use crate::task::{call_mmap, call_munmap, change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next};
use crate::mm::{translated_byte_buffer, PTEFlags, PageTable, VirtAddr};
use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    let us = get_time_us();
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, 16);
    let mut ts_ptr = &ts as *const TimeVal as *const u8;
    for buffer in buffers {
        let len = buffer.len();
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(ts_ptr, len));
            ts_ptr = ts_ptr.add(len);
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let page_table = PageTable::from_token(current_user_token());
    match _trace_request {
        0 => {
            // read
            let va= VirtAddr::from(_id);
            let pte = page_table.translate(va.floor());
            match pte {
                Some(pte) => {
                    if !pte.readable() || pte.flags() & PTEFlags::U == PTEFlags::empty() {
                        return -1;
                    }
                    let ppn = pte.ppn();
                    let address : usize = (ppn.0 << PAGE_SIZE_BITS) + (va.page_offset());
                    let data = unsafe{(address as *const u8).read_volatile()} as isize;
                    data
                }
                None => -1
            }
        }
        1 => {
            // write
            let va= VirtAddr::from(_id);
            let pte = page_table.translate(va.floor());
            match pte {
                Some(pte) => {
                    if !pte.writable() || pte.flags() & PTEFlags::U == PTEFlags::empty() {
                        return -1;
                    }
                    let ppn = pte.ppn();
                    let address : usize = (ppn.0 << PAGE_SIZE_BITS) + (va.page_offset());
                    unsafe {
                        (address as *mut u8).write_volatile(_data as u8);
                    }
                    0
                }
                None => -1
            }
        }
        2 => {
            // trace
            get_syscall_count(_id) as isize
        }
        _ => -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 || _prot & !0x7 != 0 || _prot & 0x7 == 0 {
        return -1;
    }
    call_mmap(_start, _len, _prot)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    call_munmap(_start, _len)
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
