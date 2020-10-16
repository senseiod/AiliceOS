#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(untagged_unions)]

extern crate alloc;

#[macro_use]
pub mod console;

pub mod drivers;
pub mod interrupts;
pub mod io;
pub mod lang;
pub mod memory;

use bootloader::{entry_point, BootInfo};

entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    println!("I'm from Kernel!");
    memory::heap::init_heap();
    drivers::init_driver(boot_info);
    memory::init_frame(boot_info);
    interrupts::init();
    test!("This is test");
    error!("This is error");
    warn!("This is warn");
    debug!("This is debug");
    println!("This is println");
    loop {}
}
