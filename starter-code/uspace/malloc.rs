#![allow(static_mut_refs)]
#![no_main]
#![no_std]

pub mod aux;

use aux::*;
use core::ffi::{c_void, c_char};
use core::panic::PanicInfo;
use core::fmt::Write;

// Pset Debugging
extern "C-unwind" {
    fn kernel_panic(format: *const c_char) -> !;
}


#[no_mangle]
pub fn free(_firstbyte: *mut c_void) {
    // TODO
}

#[no_mangle]
pub fn malloc(_numbytes: u64) -> *mut c_void {
    // TODO
    0 as *mut c_void
}

#[no_mangle]
pub fn calloc(_num: u64, _sz: u64) -> *mut c_void {
    // TODO
    0 as *mut c_void
}

#[no_mangle]
pub fn realloc(_ptr: *mut c_void, _sz: u64) -> *mut c_void {
    // TODO
    0 as *mut c_void
}

#[no_mangle]
pub fn defrag() {
    // TODO
}

#[no_mangle]
pub extern "C" fn heap_info(_info: *mut c_void) -> i32 {
    // TODO
    0
}


// Pset Debugging (Disregard)
// Helper function to link Rust panic with C.
// https://users.rust-lang.org/t/passing-callbacks-to-c-panic/91080/11
#[panic_handler]
#[allow(unused_must_use)]
fn panic_handler(info: &PanicInfo) -> ! {
    static mut BUFFER: [u8; 256] = [0; 256];
    let mut writer = BufferWriter::new(unsafe { &mut BUFFER });

    // Write the location if available
    if let Some(location) = info.location() {
        write!(writer, "{}:{}: ", location.file(), location.line());
    }

    // Write the message provided
    write!(writer, "{}", info.message());

    let buffer = unsafe { &mut BUFFER };
    let pos = writer.pos.min(buffer.len() - 1);
    buffer[pos] = 0;

    unsafe {
        // FFI calls are considered unsafe.
        kernel_panic(buffer.as_ptr() as *const c_char);
    }
}
