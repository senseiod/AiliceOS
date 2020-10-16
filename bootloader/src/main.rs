#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(llvm_asm)]

#[macro_use]
extern crate alloc;

mod console;
mod page_table;

use alloc::boxed::Box;
use alloc::string::ToString;
use log::*;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::{
    File, FileAttribute, FileInfo, FileMode, FileSystemVolumeLabel, RegularFile,
};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::AllocateType;
use uefi::table::cfg::ACPI2_GUID;
use x86_64::registers::control::{Cr0, Cr0Flags, Efer, EferFlags};
use xmas_elf::ElfFile;

use alloc::vec::Vec;
use bootloader::{BootInfo, GraphicInfo, KernelEntry, KernelEntryFn, MemoryType};
use core::mem;
use x86_64::structures::paging::{PageSize, Size4KiB};
use x86_64::VirtAddr;

const PHYSICAL_MEMORY_OFFSET: u64 = 0xFFFF800000000000;

#[entry]
fn efi_main(image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).expect_success("failed to initialize utilities");

    let boot_services = system_table.boot_services();
    // Initialize our "kernel" frame allocator which marks frames as `MEMORY_TYPE_KERNEL`.
    let mut frame_allocator = page_table::UEFIFrameAllocator::new(boot_services);
    let mut page_table = page_table::init_recursive(&mut frame_allocator);

    // RSDP（根系统描述指针）是ACPI编程接口中使用的数据结构。
    // 如果您使用的是UEFI，则可以在EFI_SYSTEM_TABLE中的某个位置找到它。因此，无需搜索RAM。
    let acpi2_addr = system_table
        .config_table()
        .iter()
        .find(|entry| entry.guid == ACPI2_GUID)
        .expect("Failed to find RSDP")
        .address;

    info!("ACPI2 RSDP address is : {:?}", acpi2_addr);

    // 获取memory map
    let max_mmap_size = boot_services.memory_map_size();
    let mmap_storage = Box::leak(vec![0; max_mmap_size * 2].into_boxed_slice());
    let mmap_iter = boot_services
        .memory_map(mmap_storage)
        .expect_success("failed to get memory map")
        .1;

    let max_phys_addr = mmap_iter
        .map(|m| m.phys_start + m.page_count * Size4KiB::SIZE - 1)
        .max()
        .unwrap()
        .max(0xFFFF_FFFF);

    unsafe {
        Cr0::update(|f| f.remove(Cr0Flags::WRITE_PROTECT));
        Efer::update(|f| f.insert(EferFlags::NO_EXECUTE_ENABLE));
    } // 根页表是只读的，禁用写入保护

    let (elf, kernel_entry) = load_kernel(boot_services);

    page_table::map_elf(&elf, &mut page_table, &mut frame_allocator).expect("failed to map ELF");

    page_table::map_physical_memory(
        PHYSICAL_MEMORY_OFFSET,
        max_phys_addr,
        &mut page_table,
        &mut frame_allocator,
    );

    unsafe {
        Cr0::update(|f| f.insert(Cr0Flags::WRITE_PROTECT));
    } // 恢复写入保护

    let resolution: (usize, usize) = (1280, 720);
    let graphic_info = init_graphic(boot_services, Option::from(resolution));

    // 创造一个128宽度的Vec
    // 必须在退出boot_services之前创建，否则退出后无法正常分配alloc
    let mut memory_map = Vec::with_capacity(128);

    // 退出启动服务,启动kernel
    let (_rt, mmap_iter) = system_table
        .exit_boot_services(image_handle, mmap_storage)
        .expect_success("Failed to exit boot services");

    for desc in mmap_iter {
        memory_map.push(desc);
    }

    // construct BootInfo
    let boot_info = BootInfo {
        memory_map,
        physical_memory_offset: PHYSICAL_MEMORY_OFFSET,
        graphic_info,
        acpi2_rsdp_addr: acpi2_addr as u64,
    };

    // 将bootinfo传递给内核,并跳转到内核
    jump_to_entry(&boot_info, kernel_entry);
}

fn jump_to_entry(boot_info: *const BootInfo, kernel_entry: KernelEntry) -> ! {
    println!("Jumping to Kernel ...");
    let kernel_entry: KernelEntryFn = unsafe { mem::transmute(kernel_entry) };
    kernel_entry(unsafe { &*boot_info })
}

fn load_kernel(boot_services: &BootServices) -> (ElfFile, KernelEntry) {
    let kernel_path = r"EFI\kernel\kernel.elf";
    let mut info_buf = [0u8; 0x100];
    let file_system = unsafe {
        &mut *boot_services
            .locate_protocol::<SimpleFileSystem>()
            .expect_success("Failed to open SimpleFileSystem")
            .get()
    };
    let mut root = file_system
        .open_volume()
        .expect_success("Failed to open volumes");
    let volume_label = file_system
        .open_volume()
        .expect_success("Failed to open volume")
        .get_info::<FileSystemVolumeLabel>(&mut info_buf)
        .expect_success("Failed to open volumes")
        .volume_label()
        .to_string();
    info!("The Volume Label is: {}", volume_label);
    let file_handle = root
        .open(kernel_path, FileMode::Read, FileAttribute::empty())
        .expect_success("Failed to open file");
    let mut file_handle = unsafe { RegularFile::new(file_handle) };
    info!("Loading file to memory");
    let info = file_handle
        .get_info::<FileInfo>(&mut info_buf)
        .expect_success("Failed to get file info");
    let pages = info.file_size() as usize / 0x1000 + 1;
    let mem_start = boot_services
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
        .expect_success("Failed to allocate pages");
    let buf = unsafe { core::slice::from_raw_parts_mut(mem_start as *mut u8, pages * 0x1000) };
    let len: usize = file_handle.read(buf).expect_success("Failed to read file");
    let elf = ElfFile::new(buf[..len].as_ref()).expect("Failed to parse ELF");
    let kernel_entry_addr = elf.header.pt2.entry_point();
    info!("Kernel Entry Point: {:x}", kernel_entry_addr);
    let kernel_entry: KernelEntry = KernelEntry(VirtAddr::new(kernel_entry_addr));
    // 读取内核entry地址
    (elf, kernel_entry)
}

/// If `resolution` is some, then set graphic mode matching the resolution.
/// Return information of the final graphic mode.
fn init_graphic(bs: &BootServices, resolution: Option<(usize, usize)>) -> GraphicInfo {
    let gop = bs
        .locate_protocol::<GraphicsOutput>()
        .expect_success("failed to get GraphicsOutput");
    let gop = unsafe { &mut *gop.get() };
    if let Some(resolution) = resolution {
        let mode = gop
            .modes()
            .map(|mode| mode.expect("Warnings encountered while querying mode"))
            .find(|ref mode| {
                let info = mode.info();
                info.resolution() == resolution
            })
            .expect("graphic mode not found");
        info!("switching graphic mode");
        gop.set_mode(&mode)
            .expect_success("Failed to set graphics mode");
    }
    GraphicInfo {
        mode: gop.current_mode_info(),
        fb_addr: gop.frame_buffer().as_mut_ptr() as u64,
        fb_size: gop.frame_buffer().size() as u64,
    }
}
