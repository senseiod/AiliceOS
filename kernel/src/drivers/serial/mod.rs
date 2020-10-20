use spin::Mutex;
use uart_16550::SerialPort;
use alloc::vec::Vec;
use lazy_static::*;
use alloc::collections::VecDeque;
use alloc::boxed::Box;

pub static COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });
pub static COM2: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x2F8) });


lazy_static! {
    static ref STDIN: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
    static ref STDIN_CALLBACK: Mutex<Vec<Box<dyn Fn() -> bool + Send + Sync>>> = Mutex::new(Vec::new());
}

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();
    println!("Init Serial");
}

/// Put a char by serial interrupt handler.
pub fn serial_put(mut x: u8) {
    if x == b'\r' {
        x = b'\n';
    }
    STDIN.lock().push_back(x);
    STDIN_CALLBACK.lock().retain(|f| !f());
}