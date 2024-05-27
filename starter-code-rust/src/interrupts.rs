
/////////////////////////////////////////
//   Interrupt Stack Frame Structure   //
// ----------------------------------- //
//     <------- Old Pointer            //
// Stack Allignemnt 16-byte (Variable) //
// Stack Segment (SS)                  //
// Stack Pointer                       //
// RFLAGS --------- [8 byte]           //
// Code Segment (CS)                   //
// Instruction Pointer (RIP)           //
// Error Code (Optional)               //
//     <------- New Stack Pointer      //
// Stack Frame of the Handler Function //
// ----------------------------------- //
/////////////////////////////////////////

use crate::{gdt, hlt_loop, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

use x86_64::structures::idt::{
    InterruptDescriptorTable, 
    InterruptStackFrame, 
    PageFaultErrorCode
};

// IDT vector table
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // PIC Interrupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        // Paging && VA/PA
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

// #[repr(C)]
// pub struct InterruptDescriptorTable {
//     pub divide_by_zero: Entry<HandlerFunc>,
//     pub debug: Entry<HandlerFunc>,
//     pub non_maskable_interrupt: Entry<HandlerFunc>,
//     pub breakpoint: Entry<HandlerFunc>,                    ----------- Implemented
//     pub overflow: Entry<HandlerFunc>,
//     pub bound_range_exceeded: Entry<HandlerFunc>,
//     pub invalid_opcode: Entry<HandlerFunc>,
//     pub device_not_available: Entry<HandlerFunc>,
//     pub double_fault: Entry<HandlerFuncWithErrCode>,       ----------- Implemented
//     pub invalid_tss: Entry<HandlerFuncWithErrCode>,
//     pub segment_not_present: Entry<HandlerFuncWithErrCode>,
//     pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
//     pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
//     pub page_fault: Entry<PageFaultHandlerFunc>,
//     pub x87_floating_point: Entry<HandlerFunc>,
//     pub alignment_check: Entry<HandlerFuncWithErrCode>,
//     pub machine_check: Entry<HandlerFunc>,
//     pub simd_floating_point: Entry<HandlerFunc>,
//     pub virtualization: Entry<HandlerFunc>,
//     pub security_exception: Entry<HandlerFuncWithErrCode>,
//     // some fields omitted
// }

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Handle Double and Triple Fault to avoid REBOOT
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}


//                      ____________                          ____________
// Real Time Clock --> |            |   Timer -------------> |            |
// ACPI -------------> |            |   Keyboard-----------> |            |      _____
// Available --------> | Secondary  |----------------------> | Primary    |     |     |
// Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
// Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
// Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
// Primary ATA ------> |            |   Floppy disk -------> |            |
// Secondary ATA ----> |____________|   Parallel Port 1----> |____________|

// PIC detected by the MCU
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// (Re)mmap the signal
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { 
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    });

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}


extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// Pagefault
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    // The CR2 register is automatically set by the CPU on a page fault and 
    // contains the accessed virtual address that caused the page fault.
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    hlt_loop();
}


// -------------------------------------------------------------
// Random Interrupt Test Cases

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
