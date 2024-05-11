
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

// Configurations for testing (temp)
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Unwinding panics are not supported without std
use core::panic::PanicInfo;

// Import Modules Here
mod vga_buffer;
mod serial;

// QEMU exit port and statuses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Overwriting the operating system entry point with our _start
#[no_mangle] // don't mangle (cryptic) the name of this function
pub extern "C" fn _start() {
    println!("Hello World{}", "!");


    // Run Public Tests
    #[cfg(test)]
    test_main();

    loop {}
}


// This function is called on panic
#[cfg(not(test))]
#[panic_handler] // ! means never returns
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic in test mode
// Want to quit QEMU and print to terminal
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


// Unfortunately, shutting down is relatively complex because it requires 
// implementing support for either the APM or ACPI power management standard.
// Luckily, QEMU supports a special isa-debug-exit device, which provides 
// an easy way to exit QEMU from the guest system.

// Implement a custom test framework
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

// Example
#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
