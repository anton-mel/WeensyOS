#![allow(dead_code)]

use alloc::format;
use crate::println;
use x86_64::VirtAddr;
use alloc::string::String;
use x86_64::structures::paging::OffsetPageTable;
use core::fmt::Write;  // For write! macro

const MEMSIZE_PHYSICAL: usize = 1024 * 1024 * 128;  // Example size: 128 MiB
const PAGESIZE: usize = 4096;                       // Default page size 4 KiB
const NPROC: usize = 16;                            // Example number of processes
const MEMSIZE_VIRTUAL: usize = 1024 * 1024 * 128;   // Example virtual memory size
const HZ: usize = 100;                              // Frequency for animation

static mut TICKS: usize = 0;

#[repr(u16)]
#[derive(Copy, Clone)]
enum MemStateColor {
    Kernel = 'K' as u16 | 0x0D00,
    Free = '.' as u16 | 0x0700,
    Reserved = 'R' as u16 | 0x0700,
    Shared = 'S' as u16 | 0x0F00,
}

#[derive(Copy, Clone)]
struct PageInfo {
    owner: usize,
    refcount: usize,
}

static PAGEINFO: [PageInfo; MEMSIZE_PHYSICAL / PAGESIZE] = [PageInfo { owner: 0, refcount: 0 }; MEMSIZE_PHYSICAL / PAGESIZE];

pub fn display_physical_memory() {
    let mut buffer = String::new();

    writeln!(buffer, "PHYSICAL MEMORY").unwrap();

    for pn in 0..PAGEINFO.len() {
        if pn % 64 == 0 {
            writeln!(buffer, "0x{:06X} ", pn * PAGESIZE).unwrap();
        }

        let owner = if PAGEINFO[pn].refcount == 0 { MemStateColor::Free } else { MemStateColor::Kernel };
        let _color = owner as u16;
        // Here you would use VGA buffer or similar to display with color
        // For now, just append to the buffer
        write!(buffer, "{}", owner as u8 as char).unwrap();
    }

    // Print the entire buffer at once
    println!("{}", buffer);
}

pub fn display_virtual_memory(pagetable: &OffsetPageTable<'_>, name: &str) {
    let mut buffer = String::new();

    writeln!(buffer, "VIRTUAL ADDRESS SPACE FOR {}", name).unwrap();

    for va in (0..MEMSIZE_PHYSICAL).step_by(PAGESIZE) {
        let addr = VirtAddr::new(va as u64);
        let vam = virtual_memory_lookup(pagetable, addr);
        
        let _color = if vam.is_none() {
            ' ' as u16
        } else {
            let owner = if vam.unwrap().refcount == 0 { MemStateColor::Free } else { MemStateColor::Kernel };
            let color = owner as u16;
            color
        };

        let pn = va / PAGESIZE;
        if pn % 64 == 0 {
            writeln!(buffer, "0x{:06X} ", va).unwrap();
        }
        // Here you would use VGA buffer or similar to display with color
        // For now, just append to the buffer
        write!(buffer, "{}", 'X').unwrap();  // Example placeholder
    }

    // Print the entire buffer at once
    println!("{}", buffer);
}

fn virtual_memory_lookup(_pagetable: &OffsetPageTable<'_>, _addr: VirtAddr) -> Option<PageInfo> {
    // Simulate the lookup process, returning some PageInfo for the given address
    Some(PageInfo { owner: 0, refcount: 1 }) // Example return
}

fn display_virtual_memory_animate(processes: &[Process]) {
    static mut LAST_TICKS: usize = 0;
    static mut SHOWING: usize = 1;
    const HZ: usize = 100;

    unsafe {
        if LAST_TICKS == 0 || TICKS - LAST_TICKS >= HZ / 4 {
            LAST_TICKS = TICKS;
            SHOWING += 1;
        }

        while SHOWING <= 2 * NPROC && processes[SHOWING % NPROC].state == ProcessState::Free {
            SHOWING += 1;
        }
        SHOWING = SHOWING % NPROC;

        if processes[SHOWING].state != ProcessState::Free && processes[SHOWING].display_status {
            let name = format!("{}", SHOWING);
            // Pass the reference to PageTable instead of OffsetPageTable
            display_virtual_memory(&processes[SHOWING].pagetable, &name);
        }
    }
}

fn get_ticks() -> usize {
    unsafe { TICKS }
}

struct Process<'a> {
    state: ProcessState,
    display_status: bool,
    pagetable: OffsetPageTable<'a>,
}

#[derive(PartialEq)]
enum ProcessState {
    Free,
    // other states
}
