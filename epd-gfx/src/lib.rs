const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const ROTATED: bool = true;

/// Split a framebuffer byte into two pixels of 4 bits each.
/// ```
/// assert_eq!(epd_gfx::split_byte(0xFF), (0xF, 0xF));
/// assert_eq!(epd_gfx::split_byte(0x8F), (0x8, 0xF));
/// assert_eq!(epd_gfx::split_byte(0xF0), (0xF, 0x0));
/// assert_eq!(epd_gfx::split_byte(0x00), (0x0, 0x0));
/// ```
pub fn split_byte(byte: u8) -> (u8, u8) {
    let left = (byte & 0xF0) >> 4;
    let right = byte & 0x0F;
    return (left, right);
}

pub fn draw_pixel(fb: &mut [u8], mut x: u32, mut y: u32, color: u8) {
    if ROTATED {
        let tmp = x;
        x = y;
        y = tmp;
        x = WIDTH - x - 1;
    }
    if x as u32 >= WIDTH {
        return;
    }
    if y as u32 >= HEIGHT {
        return;
    }

    let fb_index = (y * WIDTH as u32 / 2 + x / 2) as usize;
    let mut fb_byte = fb[fb_index];
    if x % 2 == 0 {
        fb_byte = (fb_byte & 0xF0) | (color >> 4);
    } else {
        fb_byte = (fb_byte & 0x0F) | (color & 0xF0);
    }
    fb[fb_index] = fb_byte;
}

pub fn set_all(fb: &mut [u8], color: u8) {
    fb.iter_mut().for_each(|x| *x = color);
}

pub fn draw_hline(fb: &mut [u8], x: u32, y: u32, length: u32, color: u8) {
    for i in 0..length {
        let xx = x + i;
        draw_pixel(fb, xx, y, color);
    }
}

pub fn draw_vline(fb: &mut [u8], x: u32, y: u32, length: u32, color: u8) {
    for i in 0..length {
        let yy = y + i;
        draw_pixel(fb, x, yy, color);
    }
}

pub fn fill_rect(fb: &mut [u8], x: u32, y: u32, w: u32, h: u32, color: u8) {
    for i in y..y + h {
        draw_hline(fb, x, i, w, color);
    }
}
