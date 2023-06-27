#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod framebuffer;
use bootloader_api::BootInfo;
use framebuffer::{Screen};

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let mut screen = Screen::new(framebuffer.buffer_mut(), info);
        
        // Clear the screen with white
        screen.clear();

        let mut intensity: u8 = 0;

        loop {
            screen.set_bg_intensity(intensity);
            intensity = intensity.wrapping_add(1);
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
