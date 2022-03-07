pub mod font;
pub mod icons;

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

fn draw_line_steep(fb: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let xi: i32;

    if dx < 0 {
        xi = -1;
        dx = -dx;
    } else {
        xi = 1;
    };

    let mut err = 2 * dx - dy;
    let mut x = x0;

    for y in y0..y1 {
        draw_pixel(fb, x as u32, y as u32, color);
        if err > 0 {
            x = x + xi;
            err = err + (2 * (dx - dy));
        } else {
            err = err + 2 * dx;
        }
    }
}

fn draw_line_shallow(fb: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let yi: i32;

    if dy < 0 {
        yi = -1;
        dy = -dy;
    } else {
        yi = 1;
    };

    let mut err = 2 * dy - dx;
    let mut y = y0;

    for x in x0..x1 {
        draw_pixel(fb, x as u32, y as u32, color);
        if err > 0 {
            y = y + yi;
            err = err + (2 * (dy - dx));
        } else {
            err = err + 2 * dy;
        }
    }
}

pub fn draw_line(fb: &mut [u8], x0: u32, y0: u32, x1: u32, y1: u32, color: u8) {
    //https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;

    if y1 == y0 {
        draw_hline(fb, x0 as u32, y0 as u32, (x1 - x0) as u32, color);
    } else if x1 == x0 {
        draw_vline(fb, x0 as u32, y0 as u32, (y1 - y0) as u32, color);
    } else {
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
                std::mem::swap(&mut y0, &mut y1);
            }
            draw_line_shallow(fb, x0, y0, x1, y1, color);
        } else {
            if y0 > y1 {
                std::mem::swap(&mut x0, &mut x1);
                std::mem::swap(&mut y0, &mut y1);
            }
            draw_line_steep(fb, x0, y0, x1, y1, color);
        }
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

pub fn fill_circle(fb: &mut [u8], u: u32, v: u32, r: u32, color: u8) {
    //https://de.wikipedia.org/wiki/Rasterung_von_Kreisen#Methode_von_Horn
    let mut x: i32 = i32::try_from(r).unwrap_or(i32::MAX);
    let mut y: i32 = 0;
    let mut d: i32 = 0; // squared distance to cirle

    while x >= y {
        // draw pixels
        let (screen_x, screen_y) = (u as i32 + x, v as i32 + y);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 - x, v as i32 - y);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 + x, v as i32 - y);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 - x, v as i32 + y);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 - y, v as i32 + x);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 + y, v as i32 - x);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 + y, v as i32 + x);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        let (screen_x, screen_y) = (u as i32 - y, v as i32 - x);
        draw_pixel(fb, screen_x as u32, screen_y as u32, color);

        // increment y and update d accordingly
        y = y + 1;
        d = d + 2 * y + 1;

        if d >= 0 {
            // if d is too large
            // increment x and update d accordingly
            x = x - 1;
            d = d - 2 * x + 1;
        }
    }
}
