#![allow(dead_code)]
use bootloader_api::{
    info::{FrameBufferInfo, PixelFormat},
    BootInfo,
};
use core::ptr;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

pub struct Screen {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    bg_intensity: u8,
}

impl Screen {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut screen = Self {
            framebuffer,
            info,
            bg_intensity: 255,
        };
        screen.clear();
        screen
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(self.bg_intensity)
    }

    pub fn width(&self) -> usize {
        self.info.width
    }

    pub fn height(&self) -> usize {
        self.info.height
    }

    pub fn set_bg_intensity(&mut self, intensity: u8) {
        self.bg_intensity = intensity;
        self.clear()
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for y in y..y + height {
            for x in x..x + width {
                self.draw_pixel(x, y, 255 - self.bg_intensity)
            }
        }
    }

    // Writes a single char to the framebuffer. Takes care of special control characters, such as
    // newlines and carriage returns.
    fn write_char(
        &mut self,
        c: char,
        x_pos: usize,
        y_pos: usize,
        weight: FontWeight,
        height: RasterHeight,
    ) {
        self.write_rendered_char(get_raster(c, weight, height).unwrap(), x_pos, y_pos);
    }

    // Prints a rendered char into the framebuffer.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar, x_pos: usize, y_pos: usize) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.draw_pixel(
                    x_pos + x,
                    y_pos + y,
                    if self.bg_intensity < 128 {
                        *byte
                    } else {
                        255 - *byte
                    },
                );
            }
        }
    }

    pub fn write_str(
        &mut self,
        s: &str,
        x_pos: usize,
        y_pos: usize,
        weight: FontWeight,
        height: RasterHeight,
    ) {
        let mut x_pos = x_pos;
        let y_pos = y_pos;
        for c in s.chars() {
            self.write_char(c, x_pos, y_pos, weight, height);
            x_pos += get_raster_width(weight, height);
        }
    }

    pub fn get_text_width(s: &str, weight: FontWeight, height: RasterHeight) -> usize {
        s.len() * get_raster_width(weight, height)
    }

    fn draw_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity, 0],
            PixelFormat::Bgr => [intensity, intensity, intensity, 0],
            PixelFormat::U8 => [intensity, 0, 0, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in screen", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

pub fn init_screen(boot_info: &'static mut BootInfo) -> Screen {
    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to get framebuffer");

    let info = framebuffer.info();
    Screen::new(framebuffer.buffer_mut(), info)
}
