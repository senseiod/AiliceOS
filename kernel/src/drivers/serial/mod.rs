use spin::Mutex;
use uart_16550::SerialPort;

pub static COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });
pub static COM2: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x2F8) });

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();
    println!("Init Serial");
}
