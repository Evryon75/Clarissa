use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SERIAL: Mutex<uart_16550::SerialPort> = {
        use uart_16550::SerialPort;
        let ser_port = unsafe {
            let mut s = SerialPort::new(0x3f8);
            s.init();
            s
        };
        Mutex::new(ser_port)
    };
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Failed to write serial");
}

#[macro_export]
macro_rules! print_serial {
    ($($arg: tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println_serial {
    () => ($crate::serial_print("\n"));
    ($fmt: expr) => ($crate::print_serial!(concat!($fmt, "\n")));
    ($fmt: expr, $($arg: tt)*) => ($crate::print_serial!(concat!($fmt, "\n"), $($arg)*));
}
