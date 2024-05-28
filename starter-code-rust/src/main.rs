
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

extern crate alloc;

use weensyos::println;
use weensyos::task::{executor::Executor, keyboard, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Rust only has a very minimal runtime, which takes care of some 
// small things such as setting up stack overflow guards or printing 
// a backtrace on panic. Still, it calls main, but our OS does not have 
// acccess to the Rust runtime, so we overwrite the operating system 
// entry point with our own _start everywhere.
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use weensyos::allocator;
    use weensyos::memory::{self, BootInfoFrameAllocator};
    // use weensyos::visual::{display_physical_memory, display_virtual_memory};
    use x86_64::VirtAddr;
    
    println!("press `{}` to exit or try typing below\n", "q");
    println!("press `{}`, `{}`, `{}`, or `{}` to load program (in dev)\n", "a", "c", "m", "t");
    weensyos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    /* display_physical_memory();
    // Initialize a temporary page table to visualize memory
    let page_table = unsafe { memory::init(phys_mem_offset) };
    display_virtual_memory(&page_table, "Kernel"); */

    // DevTests
    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::keypresses()));
    executor.run();
}


// Handle New Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Panic in Run Mode prints
    println!("{}", info);
    weensyos::fail();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Panic in Test Mode quits
    weensyos::test_panic_handler(info)
}


///////////////////////////////////////////
///
/// Some simple execution task

async fn async_number() -> u32 {
    5
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {} [ok]", number);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
