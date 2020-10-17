#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(untagged_unions)]
#![feature(llvm_asm)]
#![feature(asm)]

extern crate alloc;

#[macro_use]
pub mod console;

pub mod drivers;
pub mod interrupts;
pub mod io;
pub mod lang;
pub mod memory;
pub mod board;

use bootloader::{entry_point, BootInfo};

entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    print!(33;"\n");
    test!("I'm from Kernel!");
    memory::heap::init_heap();
    drivers::init_driver(boot_info);
    memory::init_frame(boot_info);
    interrupts::init();
    board::cpu::early_init_cpu();
    board::cpu::cpu_init();
    board::acpi::init(boot_info.acpi2_rsdp_addr as usize);
    loop {}
}
