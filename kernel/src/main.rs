#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use noto_sans_mono_bitmap::{FontWeight, RasterHeight};
use screen::Screen;

mod gdt;
mod interrupts;
mod screen;
mod serial;

bootloader_api::entry_point!(kernel_main);

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
    let mut screen = screen::init_screen(boot_info);

    gdt::init();
    interrupts::init_idt();

    draw(&mut screen);

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);
    loop {}
}
