use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{ptr};

pub struct Color(pub u8, pub u8, pub u8);

pub struct Screen {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
}

impl Screen {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            framebuffer,
            info,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [color.0, color.1, color.2, 0],
            PixelFormat::Bgr => [color.1, color.0, color.2, 0],
            PixelFormat::U8 => [(color.0 as f32 * 0.2989) as u8 + (color.1 as f32 * 0.5870) as u8 + (color.2 as f32 * 0.1140) as u8, 0, 0, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

