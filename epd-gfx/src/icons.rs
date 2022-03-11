// Symbols are drawn on a relative 10x10grid and 1 scale unit = 1 drawing unit

const BLACK: u8 = 0x0;
const WHITE: u8 = 0xF;
const LARGE: u32 = 28; // For icon drawing, needs to be odd number for best effect
const SMALL: u32 = 8; // 6  For icon drawing, needs to be odd number for best effect

use crate::drawing::{draw_hline, draw_line, draw_vline, fill_circle, fill_rect, fill_triangle};
use crate::Point;

#[derive(PartialEq, Clone, Copy)]
pub enum IconSize {
    SMALL,
    LARGE,
}

pub fn addmoon(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    if size == IconSize::LARGE {
        fill_circle(fb, x - 85, y - 100, (scale * 0.8) as i32, BLACK);
        fill_circle(fb, x - 57, y - 100, (scale * 1.6) as i32, WHITE);
    } else {
        fill_circle(fb, x - 28, y - 37, (scale * 1.0) as i32, BLACK);
        fill_circle(fb, x - 20, y - 37, (scale * 1.6) as i32, WHITE);
    }
}

// Draw a cloud of width w
pub fn cloud(fb: &mut [u8], x: i32, y: i32, w: i32) {
    let scale = w as f32 / 8.0;
    let linesize = 3i32;

    // outer circles
    let r1 = scale as i32;
    let dx_1 = w / 2 - r1;

    // top left circle
    let rt_1 = (scale * 1.4) as i32;
    let x_1 = x - scale as i32;
    let y_1 = y - scale as i32;

    // top right cirlce
    let rt_2 = (scale * 1.75) as i32;
    let x_2 = x + (scale * 1.5) as i32;
    let y_2 = y - (scale * 1.3) as i32;

    // Draw cloud outer
    fill_circle(fb, x - dx_1, y, r1, BLACK); // Left most circle
    fill_circle(fb, x + dx_1, y, r1, BLACK); // Right most circle
    fill_circle(fb, x_1, y_1, rt_1, BLACK); // left middle upper circle
    fill_circle(fb, x_2, y_2, rt_2, BLACK); // right middle upper circle
    fill_rect(fb, x - dx_1, y - r1, dx_1 * 2, r1 * 2, BLACK); // Upper and lower lines

    // Clear cloud inner
    fill_circle(fb, x - dx_1, y, r1 - linesize, WHITE);
    fill_circle(fb, x + dx_1, y, r1 - linesize, WHITE);
    fill_circle(fb, x_1, y_1, rt_1 - linesize, WHITE); // left middle upper circle
    fill_circle(fb, x_2, y_2, rt_2 - linesize, WHITE); // right middle upper circle

    fill_rect(
        fb,
        x - dx_1,
        y - r1 + linesize as i32,
        dx_1 * 2,
        2 * (r1 - linesize as i32),
        WHITE,
    );
    // Upper and lower lines
}

/// Draw a raindrop of with radius r.
fn raindrop(fb: &mut [u8], x: i32, y: i32, r: i32) {
    fill_circle(fb, x, y, r as i32, BLACK);
    fill_triangle(
        fb,
        Point { x: x - r as i32, y },
        Point {
            x,
            y: y - (2.5 * r as f32) as i32,
        },
        Point { x: x + r as i32, y },
        BLACK,
    );
}

/// Draw a row of raindrops, over width w
fn raindrops(fb: &mut [u8], x: i32, y: i32, w: i32) {
    let dx = w / 3;
    for i in -1..=1 {
        raindrop(fb, x + i * dx, y, 10);
        raindrop(fb, x + i * dx + dx / 2, y - 10, 10);
    }
}

/// Draw a snowflake of size s
pub fn snowflake(fb: &mut [u8], x: i32, y: i32, s: i32) {
    // verticals
    draw_hline(fb, x - s / 2, y, s, BLACK);
    draw_vline(fb, x, y - s / 2, s, BLACK);
    // diagonals
    let ss = (s as f32 * 0.3) as i32;
    draw_line(fb, x - ss, y - ss, x + ss, y + ss, BLACK);
    draw_line(fb, x + ss, y - ss, x - ss, y + ss, BLACK);
}

// Draw snowflakes (5) over width w
pub fn draw_snow(fb: &mut [u8], x: i32, y: i32, w: i32) {
    let dx = w / 5;
    for i in -2..=2 {
        snowflake(fb, x + i * 45, y + dx, 30);
    }
}

/// Draw fog: three horizontal lines of width w, spread across height h
pub fn draw_fog(fb: &mut [u8], x: i32, y: i32, w: i32, h: i32, linesize: u32) {
    fill_rect(fb, x - w / 2, y - h / 2, w, linesize as i32, BLACK);
    fill_rect(fb, x - w / 2, y, w, linesize as i32, BLACK);
    fill_rect(fb, x - w / 2, y + h / 2, w, linesize as i32, BLACK);
}

pub fn lightning(fb: &mut [u8], x: i32, y: i32, color: u8) {
    let h = 40; // total height
    let w = 22; // total width
    let dh = 10; // height of "middle" segment
    let p0 = Point { x, y: y - h / 2 }; // top
    let p1 = Point {
        x: x - w / 2,
        y: y + dh / 2,
    };
    let p2 = Point {
        x: x + w / 2,
        y: y - dh / 2,
    };
    let p3 = Point { x, y: y + h / 2 }; // bottom
    draw_line(fb, p0.x, p0.y, p1.x, p1.y, color);
    draw_line(fb, p1.x, p1.y, p2.x, p2.y, color);
    draw_line(fb, p2.x, p2.y, p3.x, p3.y, color);
}

pub fn addtstorm(fb: &mut [u8], x: i32, mut y: i32, scale: f32) {
    y = y + (scale / 2.0) as i32;
    for i in -1..=1 {
        lightning(fb, x + i * 45, y + 35, BLACK);
    }
}

pub fn addsun(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    if scale <= 0.0 {
        return;
    };

    let linesize = match size {
        IconSize::SMALL => 1,
        IconSize::LARGE => 3,
    };

    fill_rect(
        fb,
        x - scale as i32 * 2,
        y,
        scale as i32 * 4,
        linesize,
        BLACK,
    );
    fill_rect(
        fb,
        x,
        y - scale as i32 * 2,
        linesize,
        scale as i32 * 4,
        BLACK,
    );
    draw_line(
        fb,
        x - (scale * 1.3) as i32,
        y - (scale * 1.3) as i32,
        x + (scale * 1.3) as i32,
        y + (scale * 1.3) as i32,
        BLACK,
    );
    draw_line(
        fb,
        x - (scale * 1.3) as i32,
        y + (scale * 1.3) as i32,
        x + (scale * 1.3) as i32,
        y - (scale * 1.3) as i32,
        BLACK,
    );
    if size == IconSize::LARGE {
        draw_line(
            fb,
            1 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            1 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            2 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            2 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            3 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            3 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            1 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            1 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            2 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            2 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            3 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            3 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
    }
    fill_circle(fb, x, y, (scale * 1.3) as i32, WHITE);
    fill_circle(fb, x, y, scale as i32, BLACK);
    fill_circle(fb, x, y, scale as i32 - linesize, WHITE);
}

pub fn sunny(fb: &mut [u8], x: i32, mut y: i32, size: IconSize) {
    let scale: f32 = match size {
        IconSize::SMALL => SMALL as f32,
        IconSize::LARGE => LARGE as f32,
    };
    if size == IconSize::SMALL {
        y = y - 3; // Shift up small sun icon
    }
    addsun(fb, x, y, scale * 1.6, size);
}

pub fn mostly_sunny(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let mut offset = 5;

    let scale: f32;
    match size {
        IconSize::LARGE => {
            scale = LARGE as f32;
            offset = 10;
        }
        IconSize::SMALL => {
            scale = SMALL as f32;
        }
    }
    cloud(fb, x, y + offset, 200);
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32 + offset,
        scale,
        size,
    );
}

pub fn mostly_cloudy(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    match size {
        IconSize::LARGE => {
            scale = LARGE as f32;
        }
        IconSize::SMALL => {
            scale = SMALL as f32;
        }
    }

    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32,
        scale,
        size,
    );
    cloud(fb, x, y, 200);
}

pub fn cloudy(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    cloud(fb, x, y, 200); // Main cloud
    if size == IconSize::LARGE {
        cloud(fb, x + 30, y, 100); // Cloud top right
        cloud(fb, x - 40, y - 80, 75); // Cloud top left
    }
}

pub fn rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    cloud(fb, x, y, 200);
    raindrops(fb, x, y + 65, 200);
}

pub fn expect_rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
        }
    }
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32,
        scale,
        size,
    );
    cloud(fb, x, y, 200);
    raindrops(fb, x, y + 65, 200);
}

pub fn chance_rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    expect_rain(fb, x, y, size);
}

pub fn tstorms(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
        }
    }

    cloud(fb, x, y, 200);
    addtstorm(fb, x, y, scale);
}

pub fn snow(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
        }
    }
    cloud(fb, x, y, 200);
    draw_snow(fb, x, y + 45, scale as i32);
}

pub fn fog(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    match size {
        IconSize::SMALL => {
            panic!("fog");
        }
        IconSize::LARGE => {
            cloud(fb, x, y, 200);
            draw_fog(fb, x, y + 60, 175, 40, 3);
        }
    }
}

pub fn haze(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    match size {
        IconSize::SMALL => {
            panic!("haze");
        }
        IconSize::LARGE => {
            let scale = LARGE as f32;
            addsun(fb, x, y - 5, scale * 1.4, size);
            draw_fog(fb, x, y + 60, 175, 40, 3);
        }
    }
}
