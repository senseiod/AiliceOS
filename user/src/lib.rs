#![no_std]
#![feature(lang_items)]
#![feature(llvm_asm)]
#![feature(linkage)]

#[macro_use]
pub mod console;

pub mod lang;

use buddy_system_allocator::LockedHeap;

// 这里是程序入口
// 调用 main 函数，并利用 sys_exit 系统调用退出
#[no_mangle]
pub extern "C" fn _start(_args: isize, _argv: *const u8) {
    init_heap();
}

// 初始化用户堆
fn init_heap() {
    const HEAP_SIZE: usize = 0x1000;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, HEAP_SIZE);
    }
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();
