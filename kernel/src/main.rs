#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use noto_sans_mono_bitmap::{FontWeight, RasterHeight};
use screen::Screen;

mod screen;
mod serial;

bootloader_api::entry_point!(kernel_main);

fn init_screen(boot_info: &'static mut BootInfo) -> Screen {
    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to get framebuffer");

    let info = framebuffer.info();
    Screen::new(framebuffer.buffer_mut(), info)
}

fn draw(screen: &mut Screen) {
    let (width, _height) = (screen.width(), screen.height());

    let heading = "No operating system installed";
    let heading_width = Screen::get_text_width(heading, FontWeight::Bold, RasterHeight::Size32);

    // Draw heading
    screen.write_str(
        heading,
        width / 2 - heading_width / 2,
        100,
        FontWeight::Bold,
        RasterHeight::Size32,
    );
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Initialise screen
    let mut screen = init_screen(boot_info);

    draw(&mut screen);

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
