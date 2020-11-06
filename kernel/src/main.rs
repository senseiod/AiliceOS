#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(untagged_unions)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

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
pub mod shell;
pub mod sched;
pub mod error;
pub mod process;
pub mod config;

use bootloader::{entry_point, BootInfo};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    print!(33;"\n");
    test!("I'm from Kernel!");
    memory::heap::init_heap(); // Initialize HEAP allocation to enable alloc crate
    drivers::init_driver(boot_info); // Initialize serial output and graphic output
    board::cpu::init_cpu(); // Initialize CPU feature
    board::acpi_table::get_acpi_addr(boot_info); // Get acpi address from boot_info in Bootloader
    interrupts::init(); // Initialize Trap frame and interrupt
    memory::init_frame(boot_info); // Initialize memory Frame
    drivers::bus::pci::init(); // Initialize PCI interrupt and PCI interrupt
    fs::init(); // Initialize SFS[Simple File System]
    board::cpu::exit_in_qemu(board::cpu::QemuExitCode::Success); // Action测试需用，自己编译需注释掉
    shell::init_shell(); // call the simply shell
    unreachable!()
}