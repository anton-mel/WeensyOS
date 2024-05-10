
// To write an operating system kernel, we need code that does not depend on any 
// operating system features. This means that we can’t use threads, files, heap 
// memory, the network, random numbers, standard output, or any other features 
// requiring OS abstractions or specific hardware. Which makes sense, since we’re 
// trying to write our own OS and our own drivers

// Create an "baremetal" executable that can be run without an underlying OS
#![no_std]
// Execution starts in a C runtime library called crt0 for stack overflow
// The C runtime then invokes the entry point of the Rust runtime, also very short
// Our baremetal executable does not have access to the Rust runtime and crt0 too
#![no_main]

// Unwinding panics are not supported without std
use core::panic::PanicInfo;

// Import Modules Here
mod vga_buffer;

// Overwriting the operating system entry point with our _start
#[no_mangle] // don't mangle (cryptic) the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// This function is called on panic.
#[panic_handler] // ! means never returns
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
