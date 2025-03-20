// syscalls.c
//
//    Rust implementation of the kernel syscall helper functions.
//    Follow the kernel/c/kernel.c `exception` function for details.

use crate::*;
use crate::kernel::*;

#[no_mangle]
pub unsafe extern "C" fn syscall_pagefault(msg: *const c_char) {
    // TODO (optional)
    if msg.is_null() { return; }
    console_printf(cpos!(24, 0), 0x0C00, msg as *const u8);
    (*current).p_state = P_BROKEN;
}

#[no_mangle]
pub unsafe extern "C" fn sbrk(_p: &mut Proc, _difference: isize) -> isize {
    // TODO: Implement memory allocation logic here
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_brk() {
    // TODO
}

#[no_mangle]
pub unsafe extern "C" fn syscall_sbrk() {
    // TODO
}
