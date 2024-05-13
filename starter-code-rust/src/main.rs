
// To write an operating system kernel, we need code that does not depend on any 
// operating system features. This means that we can’t use threads, files, heap 
// memory, the network, random numbers, standard output, or any other features 
// requiring OS abstractions or specific hardware. Which makes sense, since we’re 
// trying to write our own OS and our own drivers

// Create an "baremetal" executable that 
// can be run without an underlying OS
#![no_std]
#![no_main]

// Configurations for testing (temp)
#![feature(custom_test_frameworks)]
#![test_runner(weensyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use weensyos::println;

// Rust only has a very minimal runtime, which takes care of some 
// small things such as setting up stack overflow guards or printing 
// a backtrace on panic. Still, it calls main, but our OS does not have 
// acccess to the Rust runtime, so we overwrite the operating system 
// entry point with our own _start everywhere.
#[no_mangle] // don't mangle (cryptic) the name
pub extern "C" fn _start() {
    println!("Press `{}` to exit.", "q");

    // Handle Launch Here
    weensyos::init();

    // Check VA mmap
    // I did not realize that bootloader package
    // automatically handles it already. Need to
    // fix this tomorrow morning. Check how to disable
    // or just copy booting part only inside

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // Run Public Tests
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    weensyos::hlt_loop();
}


// Handle New Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Panic in Run Mode prints
    println!("{}", info);
    weensyos::hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Panic in Test Mode quits
    weensyos::test_panic_handler(info)
}
