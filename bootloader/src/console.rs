///! 使用串口，该功能也可以在内核中使用，可直接移植
use core::fmt::{Arguments, Write};

use uart_16550::SerialPort;

pub fn print(fmt: Arguments) {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.write_fmt(fmt).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
    crate::console::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
