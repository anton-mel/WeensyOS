// To write an operating system kernel, we need code that does not depend on any
// operating system features. This means that we can’t use threads, files, heap
// memory, the network, random numbers, standard output, or any other features
// requiring OS abstractions or specific hardware. Which makes sense, since we’re
// trying to write our own OS and our own drivers.

// Create an "baremetal" executable that
// can be run without an underlying OS.
#![allow(static_mut_refs)]
#![no_main]
#![no_std]

pub mod aux;
// C headers has been translated to a Rust files
// using rust-bindgen tool that allows assert that
// that a C-Rust allignment in memory is the same.
// https://github.com/rust-lang/rust-bindgen
pub mod bindings;

// Collect all symbols and
// export as static library
// to link with C-base in toml.
pub mod kloader;
pub mod kernel;
pub mod vm;

// Rust has a minimal runtime that handles tasks such as setting up 
// stack overflow guards and printing a backtrace on panic. Writing an 
// OS in Rust allows us to avoid many segmentation faults by catching 
// issues at compile time. This eliminates the need to run the system 
// to identify certain types of crashes. However, since the current 
// implementation heavily depends on the C-based WeensyOS and uses the 
// foreign function interface (FFI), which is inherently unsafe, runtime 
// assertions remain necessary. To facilitate debugging in this assignment, 
// we need to provide a panic handler that outputs error information to 
// QEMU, similar to the behavior of the C version of WeensyOS.
use core::panic::PanicInfo;
use core::ffi::c_char;
use core::fmt::Write;
use aux::*;

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}


// Pset Debugging
extern "C-unwind" {
    /// Calls the C panic handler
    fn panic(format: *const core::ffi::c_char) -> !;
    /// Generates a formatted message using a C format string and variadic arguments
    fn generate_msg(fmt: *const core::ffi::c_char, ...) -> *const core::ffi::c_char;
    /// Debug loging to the log.txt file.
    #[allow(dead_code)]
    fn log_printf(fmt: *const core::ffi::c_char, ...);
}

// DEBUGGING PANIC
#[macro_export]
macro_rules! c_panic {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        // Null-terminate string
        let fmt_cstr = concat!($fmt, "\0");
        unsafe {
            // Call the C generate_msg(fmt, ...args)
            let msg = $crate::generate_msg(fmt_cstr.as_ptr() as *const i8, $($arg),*);
            $crate::panic(msg);
        }
    }};
}

// DEBUGGING LOG
#[macro_export]
macro_rules! c_log {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        // Null-terminate string
        let fmt_cstr = concat!($fmt, "\n\0");
        unsafe {
            // Call the C generate_msg(fmt, ...args)
            let msg = $crate::generate_msg(fmt_cstr.as_ptr() as *const i8, $($arg),*);
            log_printf(msg);
        }
    }};
}

// DO NOT USE FOR DUBUGGING (REQUIRED BY RUST)
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
        panic(buffer.as_ptr() as *const c_char);
    }
}
