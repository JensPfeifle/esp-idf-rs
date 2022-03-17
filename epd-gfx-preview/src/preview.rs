use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

const WINDOW_WIDTH: u32 = 540;
const WINDOW_HEIGHT: u32 = 960;
const BUFFER_SIZE: usize = (WINDOW_HEIGHT * WINDOW_WIDTH) as usize;
pub struct PreviewDisplay {
    width: u32,
    height: u32,
    buffer: [u8; BUFFER_SIZE],
}

impl PreviewDisplay {
    pub fn new() -> Self {
        Self {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            buffer: [0xF; BUFFER_SIZE],
        }
    }
}

impl PreviewDisplay {
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u8> {
        if let Some(idx) = self.buffer_index(x, y) {
            return Some(self.buffer[idx]);
        }
        None
    }

    fn buffer_index(&self, x: i32, y: i32) -> Option<usize> {
        // FIXME: rotation?
        if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
            return None;
        }
        let idx = y as usize * self.width as usize + x as usize;
        Some(idx)
    }
}

impl Dimensions for PreviewDisplay {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            Point { x: 0, y: 0 },
            Size {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
            },
        )
    }
}

impl DrawTarget for PreviewDisplay {
    type Color = Gray4; // 4-bit grayscale
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let Pixel(point, color) = pixel;
            let index = self.buffer_index(point.x, point.y);
            match index {
                Some(idx) => self.buffer[idx] = color.luma(),
                None => {}
            }
        }
        Ok(())
    }
}
