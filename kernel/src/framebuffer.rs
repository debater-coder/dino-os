use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::ptr;

pub struct Screen {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    bg_intensity: u8
}

impl Screen {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            framebuffer,
            info,
            bg_intensity: 255
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(255 - self.bg_intensity)
    }

    pub fn set_bg_intensity(&mut self, intensity: u8) {
        self.bg_intensity = intensity;
        self.clear()
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

