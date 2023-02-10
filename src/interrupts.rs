use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::lightredln;
use crate::vga_buffer::Color::*;
use crate::vga_buffer::ColorCode;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {

        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
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