use crate::drivers::gpu::CONSOLE;
use crate::drivers::serial::COM1;
use core::fmt::{Arguments, Write};

pub fn getchar() -> u8 {
    unsafe {
        COM1.force_unlock();
    }
    COM1.lock().receive() as u8
}

pub fn putfmt(fmt: Arguments) {
    // Out put Serial
    unsafe {
        COM1.force_unlock();
    }
    COM1.lock().write_fmt(fmt).unwrap();

    // Out put graphic
    unsafe {
        CONSOLE.force_unlock();
    }
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.write_fmt(fmt).unwrap();
    }
}

pub fn clear_screen() {
    unsafe { CONSOLE.force_unlock() }
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.clear();
    }
}
