use crate::epd_bindings;
use crate::epd_bindings::EpdiyHighlevelState;
use anyhow::Result;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;

const EPD_WIDTH: usize = 960;
const EPD_HEIGHT: usize = 540;
const FB_SIZE: usize = EPD_WIDTH / 2 * EPD_HEIGHT;

/// Split a framebuffer byte into two pixels of 4 significant bits each.
/// ```
/// assert_eq!(epd_gfx::drawing::split_byte(0xFF), (0xF, 0xF));
/// assert_eq!(epd_gfx::drawing::split_byte(0x8F), (0x8, 0xF));
/// assert_eq!(epd_gfx::drawing::split_byte(0xF0), (0xF, 0x0));
/// assert_eq!(epd_gfx::drawing::split_byte(0x00), (0x0, 0x0));
/// ```
pub fn split_byte(byte: u8) -> (u8, u8) {
    let left = byte >> 4;
    let right = byte & 0x0F;
    return (left, right);
}

/// Join two sets of 4 bits into one.
/// ```
/// assert_eq!(epd_gfx::drawing::join_bytes(0xF, 0xF),(0xFF));
/// assert_eq!(epd_gfx::drawing::join_bytes(0x8, 0xF),(0x8F));
/// assert_eq!(epd_gfx::drawing::join_bytes(0xF, 0x0),(0xF0));
/// assert_eq!(epd_gfx::drawing::join_bytes(0x0, 0x0),(0x00));
/// ```
pub fn join_bytes(left: u8, right: u8) -> u8 {
    return ((left & 0x0F) << 4) | (right & 0x0F);
}

/// Transform a point (x,y) to landscape coordinates, if possible.
/// After transformation, x will be in [0, 960) and y in [0, 540).
/// Returns None if the point lies outside of the display.
/// ```
/// assert_eq!(epd_gfx::to_landscape(50,100), Some((859,50)));
/// assert_eq!(epd_gfx::to_landscape(0,0), Some((959,0)));
/// assert_eq!(epd_gfx::to_landscape(539,959), Some((0,539)));
/// ```
pub fn to_landscape(x: i32, y: i32) -> Option<(i32, i32)> {
    if x < 0 || x >= 540 || y < 0 || y >= 960 {
        return None;
    }
    return Some((960 - y - 1, x));
}

#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}

impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}

#[derive(Debug)]
pub struct Epd {
    epdiy_state: EpdiyHighlevelState,
}

impl DrawTarget for Epd {
    type Color = Gray4; // Grayscale, 4 bits
    type Error = core::convert::Infallible; // drawing operations can never fail, since we have an internal framebuffer

    /// Draw individual pixels to the display without a defined order.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let fb = self.get_mut_buffer();
        for pixel in pixels {
            let Pixel(point, color) = pixel;
            // FIXME: rotation
            if let Some((x, y)) = to_landscape(point.x, point.y) {
                // x is [0, 960) and y is [0, 540)
                let buffer_idx: usize = ((y * 960 + x) / 2)
                    .try_into()
                    .expect("Invalid framebuffer index!");
                let (left, right) = split_byte(fb[buffer_idx]);
                // The framebuffer as understood by EPDIY has a swapped order within the bytes.
                // Example: Assuming the first two bytes of the framebuffer are 0xABCD, the
                // corresponding row of four pixels is: 0xB 0xA 0xD 0xC
                if x % 2 == 0 {
                    fb[buffer_idx] = join_bytes(left, color.luma());
                } else {
                    fb[buffer_idx] = join_bytes(color.luma(), right);
                }
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Epd {
    /// Returns the size of the bounding box.
    fn size(&self) -> Size {
        Size {
            width: EPD_WIDTH as u32,
            height: EPD_HEIGHT as u32,
        }
    }
}

impl<'a> Epd {
    pub fn new() -> Self {
        const EPD_LUT_4K: u32 = 2;
        unsafe { epd_bindings::epd_init(EPD_LUT_4K) };
        let state: EpdiyHighlevelState = unsafe { epd_bindings::epd_hl_init() };
        Self { epdiy_state: state }
    }

    // Clear the screen.
    pub fn clear(&mut self) -> () {
        unsafe { epd_bindings::epd_clear() };
    }

    /// Get a mutable slice into the  display framebuffer.
    pub fn get_mut_buffer(&mut self) -> &'a mut [u8] {
        let ptr = self.epdiy_state.front_fb;
        let fb: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, FB_SIZE) };
        fb
    }

    /// Update the screen to display the current contents of the framebuffer.
    pub fn draw(&mut self, temperature: i32) -> () {
        const MODE_GC16: epd_bindings::EpdDrawMode = 0x2;
        unsafe {
            epd_bindings::epd_poweron();
            let result: epd_bindings::EpdDrawError =
                epd_bindings::epd_hl_update_screen(&mut self.epdiy_state, MODE_GC16, temperature);
            println!("Draw result: {result:?}");
            epd_bindings::epd_poweroff();
        }
    }
}
