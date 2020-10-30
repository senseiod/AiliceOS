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

pub mod board;
pub mod drivers;
pub mod fs;
pub mod interrupts;
pub mod io;
pub mod lang;
pub mod memory;
pub mod process;
pub mod shell;

use bootloader::{entry_point, BootInfo};

entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    print!(33;"\n");
    test!("I'm from Kernel!");
    memory::heap::init_heap(); // 初始化堆分配，以启用alloc库
    drivers::init_driver(boot_info); // 初始化串口输出和显示输出
    board::cpu::init_cpu(); // 初始化CPU特性
    board::acpi_table::get_acpi_addr(boot_info); // 从 boot_info中读取acpi_table address
    interrupts::init(); // 初始化Trap frame和中断
    memory::init_frame(boot_info); // 初始化内存Frame
    drivers::bus::pci::init();
    fs::init();
    shell::init_shell();
    loop {}
}
