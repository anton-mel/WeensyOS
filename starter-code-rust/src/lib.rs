#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

// Import Modules Here
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buffer;
pub mod visual;
pub mod aux;


pub fn init() {
    gdt::init();
    aux::init_log();
    interrupts::init_idt();
    // Initialize the 8259 PIC interrups
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn fail() -> ! {
    use crate::task::{executor::Executor, keyboard, Task};

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::keypresses()));
    executor.run();
}

pub fn hlt_loop() -> ! {
    // use energy efficient loop
    loop { x86_64::instructions::hlt(); }
}

// Unfortunately, shutting down is relatively complex because it requires 
// implementing support for either the APM or ACPI power management standard.
// Luckily, QEMU supports a special isa-debug-exit device, which provides 
// an easy way to exit QEMU from the guest system.
// https://wiki.osdev.org/APM https://wiki.osdev.org/ACPI
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

/// Set-up test trait
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // Manage your template here
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Configurations for Cargo Test
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);    
    hlt_loop();
}

/// Entry point for `cargo test`
#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
