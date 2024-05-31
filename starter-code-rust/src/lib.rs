// To write an operating system kernel, we need code that does not depend on any 
// operating system features. This means that we can’t use threads, files, heap 
// memory, the network, random numbers, standard output, or any other features 
// requiring OS abstractions or specific hardware. Which makes sense, since we’re 
// trying to write our own OS and our own drivers

// Create an "baremetal" executable that 
// can be run without an underlying OS
#![no_std]
#![cfg_attr(test, no_main)]

#![feature(lang_items)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]

// Configurations for testing (temp)
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;

// Import Modules
use core::panic::PanicInfo;

// Rust only has a very minimal runtime, which takes care of some 
// small things such as setting up stack overflow guards or printing 
// a backtrace on panic. Still, it calls main, but our OS does not have 
// acccess to the Rust runtime, so we overwrite the operating system 
// entry point with our own _start everywhere.
#[no_mangle]
pub extern "C" fn kernel_entry() {
    // ATTENTION: we have a very small stack and no guard page
    hlt_loop();
}

// Handle New Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Panic in Run Mode prints
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Panic in Test Mode quits
    test_panic_handler(info)
}

// Energy-efficient endless loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

// https://wiki.osdev.org/APM https://wiki.osdev.org/ACPI
// Unfortunately, shutting down is relatively complex because it requires 
// implementing support for either the APM or ACPI power management standard.
// Luckily, QEMU supports a special isa-debug-exit device, which provides 
// an easy way to exit QEMU from the guest system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::PortWriteOnly;
    const PORT_EXIT: u16 = 0xf4;

    unsafe {
        let mut port = PortWriteOnly::new(PORT_EXIT);
        port.write(exit_code as u32);
    }
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}
