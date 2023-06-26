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
        
        // Clear the screen with white
        screen.clear_grayscale(255);

        // Draw a black pixel at (0, 0)
        screen.draw_pixel(0, 0, Color(0, 0, 0));

        // Draw a red line

        screen.draw_line(100, 100, 300, 300, Color(255, 0, 0))
    }
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
