use crate::memory::HEAP_ALLOCATOR;
use core::mem;

pub const KERNEL_HEAP_SIZE: usize = 8 * 1024 * 1024;
const MACHINE_ALGIN: usize = mem::size_of::<usize>();
const HEAP_BLOCK: usize = KERNEL_HEAP_SIZE / MACHINE_ALGIN;
static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];

/// # Safety
/// Buddy_system_allocator's init() method is unsafety.
/// Because init() method can only be called once to initialize the HEAP.
/// So we use unsafe code.
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, HEAP_BLOCK * MACHINE_ALGIN);
    }
    test!("Init Heap");
}
