use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{ptr};

#[derive(Copy, Clone)]
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

    pub fn clear_grayscale(&mut self, intensity: u8) {
        self.framebuffer.fill(intensity)
    }

    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color) {
        // #CopyPasta
        let steep = (y2 - y1).abs() > (x2 - x1).abs();
        let (x0, mut y0) = if steep {
            (y1, x1)
        } else {
            (x1, y1)
        };
        let (x3, y3) = if steep {
            (y2, x2)
        } else {
            (x2, y2)
        };
        let dx = x3 - x0;
        let dy = (y3 - y0).abs();
        let mut error = dx / 2;
        let ystep = if y0 < y3 { 1 } else { -1 };
        for x in x0..=x3 {
            self.draw_pixel(if steep { y0.try_into().unwrap() } else { x.try_into().unwrap() }, if steep {x.try_into().unwrap()} else {y0.try_into().unwrap()}, color);
            error -= dy;
            if error < 0 {
                y0 += ystep;
                error += dx;
            }
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [color.0, color.1, color.2, 0],
            PixelFormat::Bgr => [color.2, color.1, color.0, 0],
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

