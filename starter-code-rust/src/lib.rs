// Compiled As WeenseOS Library
// Current Implementation Includes
// 1. VGA text mode, println and print
// 2. Panic supported by VGA buffer
// 3. Testable Trait Set-up
// 4. Interrupts

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

// Import Modules Here
pub mod vga_buffer;
pub mod interrupts;
pub mod serial;             // terminal route
pub mod gdt;


pub fn init() {
    gdt::init();
    interrupts::init_idt();
    // Initialize the 8259 PIC interrups
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); 
}

// energy-efficient endless loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

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
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Set-up test trait
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
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
