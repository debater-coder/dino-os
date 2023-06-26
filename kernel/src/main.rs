#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod framebuffer;
use bootloader_api::BootInfo;
use framebuffer::{Screen, Color};

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let mut screen = Screen::new(framebuffer.buffer_mut(), info);
        
        for x in 0..info.width-1 {
            for y in 0..info.height-1 {

                screen.draw_pixel(x, y, Color(x as u8, y as u8, 128))
            }
        }
    }
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
