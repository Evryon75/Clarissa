use crate::lightredln;
use crate::vga_buffer::Color::*;
use crate::vga_buffer::ColorCode;
use crate::{gdt, magenta};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, ScancodeSet1, DecodedKey};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::hlt;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub fn initialize_idt() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = 40;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn page_fault_handler(stack: InterruptStackFrame, error: PageFaultErrorCode) {
    lightredln!("Exception: Page Fault");
    lightredln!("Accessed address: {:?}", Cr2::read());
    lightredln!("Accessed code: {:?}", error);
    lightredln!("Stack frame: {:#?}", stack);
    hlt();
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack: InterruptStackFrame) {
    use pc_keyboard::Keyboard;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Uk105Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();

    let scancode: u8 = unsafe {
        Port::new(0x60).read()
    };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(k) => magenta!("{:?}", k),
                DecodedKey::Unicode(char) => magenta!("{}", char)
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception: Double fault\n{:#?}", stack);
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    lightredln!("Exception: Breakpoint\n{:#?}", stack);
}

#[test_case]
fn breakpoint_test() {
    x86_64::instructions::interrupts::int3();
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
