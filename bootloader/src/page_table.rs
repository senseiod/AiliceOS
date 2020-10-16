//! 该文件是从'rust-osdev/bootloader'中的'page_table.rs'修改而来的
use log::*;
use uefi::table::boot::{AllocateType, BootServices, MemoryType};
use uefi::ResultExt;
use x86_64::registers::control::{Cr3, Cr3Flags, Cr4, Cr4Flags, Efer, EferFlags};
use x86_64::structures::paging::{
    mapper::*, FrameAllocator, Mapper, Page, PageSize, PageTable, PageTableFlags, PhysFrame,
    Size2MiB, Size4KiB,
};
use x86_64::{align_up, PhysAddr, VirtAddr};
use xmas_elf::{program, ElfFile};

pub struct UEFIFrameAllocator<'a>(&'a BootServices);

impl<'a> UEFIFrameAllocator<'a> {
    pub fn new(services: &'a BootServices) -> Self {
        Self(services)
    }
}

unsafe impl<'a> FrameAllocator<Size4KiB> for UEFIFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let phys_addr = self
            .0
            .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)
            .expect_success("Failed to allocate physical frame");
        let phys_addr = PhysAddr::new(phys_addr);
        let phys_frame = PhysFrame::containing_address(phys_addr);
        Some(phys_frame)
    }
}

pub fn map_elf(
    elf: &ElfFile,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    info!("mapping ELF");
    let kernel_start = PhysAddr::new(elf.input.as_ptr() as u64);
    for segment in elf.program_iter() {
        map_segment(&segment, kernel_start, page_table, frame_allocator)?;
    }
    Ok(())
}

fn map_segment(
    segment: &program::ProgramHeader,
    kernel_start: PhysAddr,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    if let program::Type::Load = segment.get_type().unwrap() {
        let mem_size = segment.mem_size();
        let file_size = segment.file_size();
        let file_offset = segment.offset() & !0xfff;
        let phys_start_addr = kernel_start + file_offset;
        let virt_start_addr = VirtAddr::new(segment.virtual_addr());

        let start_page: Page = Page::containing_address(virt_start_addr);
        let start_frame = PhysFrame::containing_address(phys_start_addr);
        let end_frame = PhysFrame::containing_address(phys_start_addr + file_size - 1u64);

        let flags = segment.flags();
        let mut page_table_flags = PageTableFlags::PRESENT;
        if !flags.is_execute() {
            page_table_flags |= PageTableFlags::NO_EXECUTE
        };
        if flags.is_write() {
            page_table_flags |= PageTableFlags::WRITABLE
        };

        for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
            let offset = frame - start_frame;
            let page = start_page + offset;
            unsafe {
                page_table
                    .map_to(page, frame, page_table_flags, frame_allocator)?
                    .flush();
            }
        }

        if mem_size > file_size {
            // .bss section (or similar), which needs to be zeroed
            let zero_start = virt_start_addr + file_size;
            let zero_end = virt_start_addr + mem_size;
            if zero_start.as_u64() & 0xfff != 0 {
                // A part of the last mapped frame needs to be zeroed. This is
                // not possible since it could already contains parts of the next
                // segment. Thus, we need to copy it before zeroing.

                let new_frame = frame_allocator
                    .allocate_frame()
                    .ok_or(MapToError::FrameAllocationFailed)?;

                type PageArray = [u64; Size4KiB::SIZE as usize / 8];

                let last_page = Page::containing_address(virt_start_addr + file_size - 1u64);
                let last_page_ptr = end_frame.start_address().as_u64() as *mut PageArray;
                let temp_page_ptr = new_frame.start_address().as_u64() as *mut PageArray;

                unsafe {
                    // copy contents
                    temp_page_ptr.write(last_page_ptr.read());
                }

                // remap last page
                if let Err(e) = page_table.unmap(last_page) {
                    return Err(match e {
                        UnmapError::ParentEntryHugePage => MapToError::ParentEntryHugePage,
                        UnmapError::PageNotMapped => unreachable!(),
                        UnmapError::InvalidFrameAddress(_) => unreachable!(),
                    });
                }
                unsafe {
                    page_table
                        .map_to(last_page, new_frame, page_table_flags, frame_allocator)?
                        .flush();
                }
            }

            // Map additional frames.
            let start_page: Page = Page::containing_address(VirtAddr::new(align_up(
                zero_start.as_u64(),
                Size4KiB::SIZE,
            )));
            let end_page = Page::containing_address(zero_end);
            for page in Page::range_inclusive(start_page, end_page) {
                let frame = frame_allocator
                    .allocate_frame()
                    .ok_or(MapToError::FrameAllocationFailed)?;
                unsafe {
                    page_table
                        .map_to(page, frame, page_table_flags, frame_allocator)?
                        .flush();
                }
            }

            // zero bss
            unsafe {
                core::ptr::write_bytes(
                    zero_start.as_mut_ptr::<u8>(),
                    0,
                    (mem_size - file_size) as usize,
                );
            }
        }
    }
    Ok(())
}

/// 将物理内存[0, max_addr]映射到虚拟空间[offset, offset + max_addr]
pub fn map_physical_memory(
    offset: u64,
    max_addr: u64,
    page_table: &mut impl Mapper<Size2MiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    info!("mapping physical memory");
    let start_frame = PhysFrame::containing_address(PhysAddr::new(0));
    let end_frame = PhysFrame::containing_address(PhysAddr::new(max_addr));
    for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
        let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64() + offset));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            page_table
                .map_to(page, frame, flags, frame_allocator)
                .expect("failed to map physical memory")
                .flush();
        }
    }
}

/// Set up a basic recursive page table.
pub fn init_recursive(
    allocator: &mut impl FrameAllocator<Size4KiB>,
) -> RecursivePageTable<'static> {
    // First we do a copy for the level 4 table here, because the old table
    // has memory type `BOOT_SERVICES_DATA`. Level 3 ~ level 1 tables will
    // be discarded eventually so we can ignore them.
    let old_l4_table_addr = Cr3::read().0.start_address().as_u64();
    let l4_table_frame = allocator.allocate_frame().unwrap();
    let l4_table_addr = l4_table_frame.start_address().as_u64();

    // Safety: newly allocated frame is guaranteed to be valid and unused
    unsafe {
        core::ptr::copy(
            old_l4_table_addr as *const u8,
            l4_table_addr as *mut u8,
            l4_table_frame.size() as usize,
        )
    };

    // Safety: same as above
    let l4_table = unsafe { &mut *(l4_table_addr as *mut PageTable) };

    // Recursive mapping
    l4_table[0b111_111_111].set_frame(
        l4_table_frame,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
    );

    // Enable all CPU extensions we need.
    unsafe {
        Cr4::update(|cr4| {
            cr4.insert(
                Cr4Flags::PAGE_SIZE_EXTENSION
                    | Cr4Flags::PHYSICAL_ADDRESS_EXTENSION
                    | Cr4Flags::PAGE_GLOBAL
                    | Cr4Flags::OSFXSR,
            )
        });
        Efer::update(|efer| efer.insert(EferFlags::NO_EXECUTE_ENABLE));
    };

    // Switch to the new page table...
    unsafe { Cr3::write(l4_table_frame, Cr3Flags::empty()) };

    // And we have it!
    let l4_table = unsafe { &mut *(0xFFFF_FFFF_FFFF_F000 as *mut PageTable) };

    RecursivePageTable::new(l4_table).unwrap()
}
