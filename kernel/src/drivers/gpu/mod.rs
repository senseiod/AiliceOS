use rcore_console::{DrawTarget, Rgb888, Pixel, Size, Console, ConsoleOnGraphic};
use bootloader::BootInfo;
use crate::memory::phys_to_virt;
use spin::Mutex;

pub static CONSOLE: Mutex<Option<ConsoleOnGraphic<Framebuffer>>> = Mutex::new(None);

pub struct Framebuffer {
    width: usize,
    height: usize,
    buf: &'static mut [u32],
}

impl DrawTarget<Rgb888> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, item: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let idx = item.0.x as usize + item.0.y as usize * self.width;
        self.buf[idx] = unsafe { core::mem::transmute(item.1) };
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

pub fn init_graphic_framebuffer(boot_info: &BootInfo) {
    let (width, height) = boot_info.graphic_info.mode.resolution();
    let fb_addr =  boot_info.graphic_info.fb_addr as usize;
    let fb = Framebuffer {
        width,
        height,
        buf: unsafe {
            core::slice::from_raw_parts_mut(
                phys_to_virt(fb_addr) as *mut u32,
                (width * height) as usize,
            )
        },
    };
    let console = Console::on_frame_buffer(fb);
    *CONSOLE.lock() = Some(console);
    println!("Init Graphic framebuffer");
}