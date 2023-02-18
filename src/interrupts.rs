use crate::gdt;
use crate::lightredln;
use crate::vga_buffer::Color::*;
use crate::vga_buffer::ColorCode;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

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

extern "x86-interrupt" fn double_fault_handler(
    stack_f: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("Exception: Double fault\n{:#?}", stack_f);
}

pub fn initialize_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    lightredln!("Exception: Breakpoint\n{:#?}", stack);
}

#[test_case]
fn breakpoint_test() {
    x86_64::instructions::interrupts::int3();
}
