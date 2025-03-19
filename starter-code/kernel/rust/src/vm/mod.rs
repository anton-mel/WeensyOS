// This file is for linking C vm functionality with Rust.
// For better understanding of FFI consider reading this documentation.
// https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions

pub mod vm;

use crate::bindings::bindings_x86_64::*;
use crate::bindings::bindings_kernel::*;

extern "C-unwind" {
    pub static kernel_pagetable: *mut x86_64_pagetable;
}

extern "C-unwind" {
    pub fn lookup_l1pagetable(
        pagetable: *mut x86_64_pagetable, // Pointer to the page table
        va: usize,                        // Virtual address
        perm: i32,                        // Permissions
    ) -> *mut x86_64_pagetable;
}
