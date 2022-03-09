use crate::Point;

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

pub fn draw_pixel(fb: &mut [u8], x: i32, y: i32, color: u8) {
    // EPD expects framebuffer for landscape display (WIDTH > HEIGHT)
    if let Some((x, y)) = to_landscape(x, y) {
        // x is [0, 960) and y is [0, 540)
        let fb_index: usize = ((y * 960 + x) / 2)
            .try_into()
            .expect("Invalid framebuffer index!");
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

pub fn draw_hline(fb: &mut [u8], x: i32, y: i32, length: i32, color: u8) {
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
        draw_pixel(fb, x, y, color);
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
        draw_pixel(fb, x, y, color);
        if err > 0 {
            y = y + yi;
            err = err + (2 * (dy - dx));
        } else {
            err = err + 2 * dy;
        }
    }
}

pub fn draw_line(fb: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
    //https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    if y1 == y0 {
        draw_hline(fb, x0, y0, x1 - x0, color);
    } else if x1 == x0 {
        draw_vline(fb, x0, y0, y1 - y0, color);
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

pub fn draw_vline(fb: &mut [u8], x: i32, y: i32, length: i32, color: u8) {
    for i in 0..length {
        let yy = y + i;
        draw_pixel(fb, x, yy, color);
    }
}

pub fn fill_rect(fb: &mut [u8], x: i32, y: i32, w: i32, h: i32, color: u8) {
    for i in y..y + h {
        draw_hline(fb, x, i, w, color);
    }
}

pub fn fill_circle(fb: &mut [u8], u: i32, v: i32, r: i32, color: u8) {
    if r < 0 {
        return;
    };
    let r2 = r * r;
    let area = r2 << 2;
    let rr = r << 1;

    for i in 0..area {
        let tx = (i % rr) - r;
        let ty = (i / rr) - r;

        if tx * tx + ty * ty <= r2 {
            draw_pixel(fb, u + tx, v + ty, color);
        }
    }
}

///https://de.wikipedia.org/wiki/Rasterung_von_Kreisen#Methode_von_Horn
pub fn draw_circle(fb: &mut [u8], u: i32, v: i32, r: i32, color: u8) {
    if r < 0 {
        return;
    };
    let mut x = r;
    let mut y = 0;
    let mut d = 0; // squared distance to cirle

    while x >= y {
        // draw pixels
        draw_pixel(fb, u + x, v + y, color);
        draw_pixel(fb, u - x, v + y, color);
        draw_pixel(fb, u + x, v - y, color);
        draw_pixel(fb, u - x, v - y, color);
        draw_pixel(fb, v + x, u + y, color);
        draw_pixel(fb, v - x, u + y, color);
        draw_pixel(fb, v + x, u - y, color);
        draw_pixel(fb, v - x, u - y, color);

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

pub fn draw_triangle(fb: &mut [u8], p0: Point, p1: Point, p2: Point, color: u8) {
    draw_line(fb, p0.x, p0.y, p1.x, p1.y, color);
    draw_line(fb, p1.x, p1.y, p2.x, p2.y, color);
    draw_line(fb, p2.x, p2.y, p0.x, p0.y, color);
}

/// This has a major limitation - can only handle upward pointing triangles at the moment!
pub fn fill_triangle(fb: &mut [u8], mut p0: Point, mut p1: Point, mut p2: Point, color: u8) {
    // Sort coordinates so that x2 >= x1 >= x0
    if p0.x > p1.x {
        std::mem::swap(&mut p0, &mut p1);
    }
    if p1.x > p2.x {
        std::mem::swap(&mut p2, &mut p1);
    }
    if p0.x > p1.x {
        std::mem::swap(&mut p0, &mut p1);
    }

    // FIXME: Handle all points on one line?
    if !(p2.x > p1.x && p1.x > p0.x) {
        panic!("fill_triangle: sorting points by rising x-coordinate failed!")
    }

    if p0.y != p2.y || p1.y >= p0.y {
        panic!("fill_triangle: can only handle upward-pointing triangles!")
    }

    // Assuming triangle looks like this:
    //  /\
    // /  \
    // ----

    // Starting from the top
    let mut l: f32 = 0.0;
    let m = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32; // m = dy/dx
    for y in p1.y..p2.y {
        let start = p1.x - l as i32;
        let len = (l * 2.0) as i32;
        draw_hline(fb, start, y, len, color);
        l = l + (1.0 / m);
    }
}
