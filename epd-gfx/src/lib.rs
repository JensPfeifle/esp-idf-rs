pub mod font;

/// Split a framebuffer byte into two pixels of 4 significant bits each.
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

/// Join two sets of 4 bits into one.
/// ```
/// assert_eq!(epd_gfx::join_bytes(0xF, 0xF),(0xFF));
/// assert_eq!(epd_gfx::join_bytes(0x8, 0xF),(0x8F));
/// assert_eq!(epd_gfx::join_bytes(0xF, 0x0),(0xF0));
/// assert_eq!(epd_gfx::join_bytes(0x0, 0x0),(0x00));
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
pub fn to_landscape(x: u32, y: u32) -> Option<(u32, u32)> {
    if x >= 540 || y >= 960 {
        return None;
    }
    return Some((960 - y - 1, x));
}

pub fn draw_pixel(fb: &mut [u8], x: u32, y: u32, color: u8) {
    // EPD expects framebuffer for landscape display (WIDTH > HEIGHT)
    if let Some((x, y)) = to_landscape(x, y) {
        // x is [0, 960) and y is [0, 540)
        let fb_index = ((y * 960 + x) / 2) as usize;
        let (left, right) = split_byte(fb[fb_index]);
        if x % 2 == 0 {
            fb[fb_index] = join_bytes(left, color);
        } else {
            fb[fb_index] = join_bytes(color, right);
        }
    }
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
