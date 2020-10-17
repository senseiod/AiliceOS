use bootloader::BootInfo;

pub mod gpu;
pub mod serial;

pub fn init_driver(boot_info: &BootInfo) {
    serial::init();
    gpu::init_graphic_framebuffer(boot_info);
    test!("Init Drivers");
}
