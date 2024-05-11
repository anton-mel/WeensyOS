
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

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{gdt, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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
//     pub double_fault: Entry<HandlerFuncWithErrCode>,
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


// -------------------------------------------------------------

#[cfg(test)]
use crate::{serial_print, serial_println};

// Random Interrupt test cases
#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}