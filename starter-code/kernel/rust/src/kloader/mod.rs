// YOU DO NOT NEED TO UNDERTAND THIS FILE.
// 
// This file is for linking C k-loader functionality with Rust.
// For better understanding of FFI consider reading this documentation.
// https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions

pub mod kloader;

use crate::bindings::bindings_elf::*;
use crate::bindings::bindings_x86_64::*;

extern "C-unwind" {
    pub static kernel_pagetable: *mut x86_64_pagetable;
}

extern "C-unwind" {
    pub fn set_pagetable(pagetable: *mut x86_64_pagetable);
    pub fn assign_physical_page(addr: usize, owner: i8) -> core::ffi::c_int;
    pub fn roundup(a: usize, n: usize) -> usize;
    pub fn virtual_memory_lookup(
        pagetable: *mut x86_64_pagetable, // Pointer to the page table
        va: usize,                        // Virtual address
    ) -> VAMapping;
    pub fn virtual_memory_map(
        pagetable: *mut x86_64_pagetable, // Pointer to the page table
        va: usize,                        // Virtual address
        pa: usize,                        // Physical address
        sz: usize,                        // Size
        perm: i32,                        // Permissions
    ) -> i32;
    pub fn console_printf(
        cpos: i32,
        color: i32,
        format: *const u8,
        ...
    ) -> i32;
}
