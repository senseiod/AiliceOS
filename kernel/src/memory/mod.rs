use bitmap_allocator::BitAlloc;
use bootloader::{BootInfo, MemoryType};
use buddy_system_allocator::LockedHeap;
use lazy_static::*;
use spin::Mutex;

pub const PAGE_SIZE: usize = 1 << 12;
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xffff8000_00000000;
pub const KERNEL_OFFSET: usize = 0xffffff00_00000000;

pub mod heap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub type FrameAlloc = bitmap_allocator::BitAlloc256M;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAlloc> = Mutex::new(FrameAlloc::default());
}

pub fn init_frame(boot_info: &BootInfo) {
    let mut all_size: f64 = 0.0;
    let memory_map = &boot_info.memory_map;
    println!(
        "Kernel usable memory range ({} total)",
        memory_map.clone().iter().len()
    );
    for descriptor in memory_map.clone().iter() {
        if descriptor.ty == MemoryType::CONVENTIONAL {
            let start_frame = descriptor.phys_start as usize / PAGE_SIZE;
            let end_frame = start_frame + descriptor.page_count as usize;
            println!(
                "{:#x} - {:#x} ({} Kib)",
                start_frame, end_frame, descriptor.page_count
            );
            all_size += descriptor.page_count as f64;
            FRAME_ALLOCATOR.lock().insert(start_frame..end_frame);
        }
    }
    let all_size = all_size * PAGE_SIZE as f64 / (1024 * 1024) as f64;
    println!("Total memory: {} M", all_size);
    println!("Init memory Frame");
}

/// Convert physical address to virtual address
#[inline]
pub const fn phys_to_virt(paddr: usize) -> usize {
    PHYSICAL_MEMORY_OFFSET + paddr
}

/// Convert virtual address to physical address
#[inline]
pub const fn virt_to_phys(vaddr: usize) -> usize {
    vaddr - PHYSICAL_MEMORY_OFFSET
}

/// Convert virtual address to the offset of kernel
#[inline]
pub const fn kernel_offset(vaddr: usize) -> usize {
    vaddr - KERNEL_OFFSET
}
