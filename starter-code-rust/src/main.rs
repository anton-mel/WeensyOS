
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

use weensyos::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;


// Rust only has a very minimal runtime, which takes care of some 
// small things such as setting up stack overflow guards or printing 
// a backtrace on panic. Still, it calls main, but our OS does not have 
// acccess to the Rust runtime, so we overwrite the operating system 
// entry point with our own _start everywhere.
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use weensyos::kernel::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    // Handle Launch Here
    println!("Press `{}` to exit.", "q");
    weensyos::init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };


    // Check logging
    use log::warn;
    warn!("works!");

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
