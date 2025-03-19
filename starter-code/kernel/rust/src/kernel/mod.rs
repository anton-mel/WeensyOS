// This file is for linking C kernel functionality with Rust.
// For better understanding of FFI consider reading this documentation.
// https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions

pub mod kernel;

use crate::kernel::kernel::PhysicalPageInfo;
use crate::bindings::bindings_x86_64::*;
use crate::bindings::bindings_kernel::*;

extern "C-unwind" {
    pub static mut ticks: u32;
    pub static mut disp_global: u8;
    pub static mut current: *mut Proc;
    pub static mut cursorpos: core::ffi::c_int;
    pub static mut pageinfo: [PhysicalPageInfo; NPAGES as usize];
    pub static mut console: [u16; CONSOLE_ROWS * CONSOLE_COLUMNS];
    pub static kernel_pagetable: *mut x86_64_pagetable;
    pub static mut processes: [Proc; NPROC];
}

extern "C-unwind" {
    pub fn hardware_init();
    pub fn pageinfo_init();
    pub fn console_clear();
    pub fn timer_init(hz: u32);
    pub fn run(p: &mut Proc);
    pub fn schedule();
    pub fn asm_rcr2() -> u64;
    pub fn assign_physical_page(addr: usize, owner: i8) -> core::ffi::c_int;
    pub fn program_load(process: *mut Proc, program_number: i32, arg: *const u8) -> i32;
    pub fn process_init(process: *mut Proc, flag: usize);
    pub fn virtual_memory_map(
        pagetable: *mut x86_64_pagetable, // Pointer to the page table
        va: usize,                        // Virtual address
        pa: usize,                        // Physical address
        sz: usize,                        // Size
        perm: i32,                        // Permissions
    ) -> i32;
    pub fn virtual_memory_lookup(
        pagetable: *mut x86_64_pagetable, // Pointer to the page table
        va: usize,                        // Virtual address
    ) -> VAMapping;
    pub fn syscall_mapping(p: &mut Proc);
    pub fn syscall_mem_tog(p: &mut Proc);
    pub fn check_keyboard() -> core::ffi::c_int;
    pub fn console_show_cursor(cpos: core::ffi::c_int);
    pub fn set_pagetable(pagetable: *mut x86_64_pagetable);
    pub fn default_exception(p: *mut Proc);
    pub fn memshow_virtual_animate();
    pub fn check_virtual_memory();
    pub fn memshow_physical();
    pub fn memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    pub fn strcmp(
        a: *const core::ffi::c_char,
        b: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn console_printf(
        cpos: i32,
        color: i32,
        format: *const u8,
        ...
    ) -> i32;
}
