// On stack overflow, when a page fault occurs, the CPU looks up the page fault handler 
// in the IDT and tries to push the interrupt stack frame onto the stack. 
// However, the current stack pointer still points to the non-present guard page. 
// Thus, a second page fault occurs, which causes a double fault. So the CPU tries to 
// call the double fault handler now. However, on a double fault, the CPU tries to push 
// the exception stack frame, too. The stack pointer still points to the guard page, so 
// a third page fault occurs, which causes a triple fault and a system reboot.

// The Interrupt Stack Table (IST) is part of an old legacy structure called 
// Task State Segment (TSS). Here we create a new TSS that contains a separate double 
// fault stack in its interrupt stack table to handle this issue.

use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;
use core::ptr::addr_of;
use x86_64::VirtAddr;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
