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

pub fn addcloud(fb: &mut [u8], x: i32, y: i32, scale: f32, linesize: u32) {
    // Draw cloud outer
    fill_circle(fb, x - scale as i32 * 3, y, scale as i32, BLACK); // Left most circle
    fill_circle(fb, x + scale as i32 * 3, y, scale as i32, BLACK); // Right most circle
    fill_circle(
        fb,
        x - scale as i32,
        y - scale as i32,
        (scale * 1.4) as i32,
        BLACK,
    ); // left middle upper circle
    fill_circle(
        fb,
        x + (scale * 1.5) as i32,
        y - (scale * 1.3) as i32,
        (scale * 1.75) as i32,
        BLACK,
    ); // Right middle upper circle
    fill_rect(
        fb,
        x - scale as i32 * 3 - 1,
        y - scale as i32,
        scale as i32 * 6,
        scale as i32 * 2 + 1,
        BLACK,
    ); // Upper and lower lines
       // Clear cloud inner
    fill_circle(
        fb,
        x - scale as i32 * 3,
        y,
        (scale - linesize as f32) as i32,
        WHITE,
    ); // Clear left most circle
    fill_circle(
        fb,
        x + scale as i32 * 3,
        y,
        (scale - linesize as f32) as i32,
        WHITE,
    ); // Clear right most circle
    fill_circle(
        fb,
        x - scale as i32,
        y - scale as i32,
        ((scale * 1.4) - (linesize as f32)) as i32,
        WHITE,
    ); // left middle upper circle
    fill_circle(
        fb,
        x + (scale * 1.5) as i32,
        y - (scale * 1.3) as i32,
        ((scale * 1.75) - (linesize as f32)) as i32,
        WHITE,
    ); // Right middle upper circle
    fill_rect(
        fb,
        x - scale as i32 * 3 + 2,
        y - scale as i32 + linesize as i32 - 1,
        (scale * 5.9) as i32,
        ((scale * 2.0) - (linesize as f32) + 2.0) as i32,
        WHITE,
    ); // Upper and lower lines
}

pub fn addraindrop(fb: &mut [u8], mut x: i32, mut y: i32, scale: f32) {
    fill_circle(fb, x, y, scale as i32 / 2, BLACK);
    fill_triangle(
        fb,
        Point {
            x: x - (scale / 2.0) as i32,
            y,
        },
        Point {
            x,
            y: y - (scale * 1.2) as i32,
        },
        Point {
            x: x + (scale / 2.0) as i32,
            y,
        },
        BLACK,
    );
    x = x + (scale * 1.6) as i32;
    y = y + (scale / 3.0) as i32;
    fill_circle(fb, x, y, (scale / 2.0) as i32, BLACK);
    fill_triangle(
        fb,
        Point {
            x: x - (scale / 2.0) as i32,
            y,
        },
        Point {
            x,
            y: y - (scale * 1.2) as i32,
        },
        Point {
            x: x + (scale / 2.0) as i32,
            y,
        },
        BLACK,
    );
}

pub fn addrain(fb: &mut [u8], x: i32, y: i32, mut scale: f32, size: IconSize) {
    if size == IconSize::SMALL {
        scale = scale * 1.34;
    }
    for d in 0..4 {
        addraindrop(
            fb,
            x + (scale * (7.8 - d as f32 * 1.95) - (scale * 5.2)) as i32,
            y + (scale * 2.1 - scale / 6.0) as i32,
            scale / 1.6,
        );
    }
}

pub fn snowflake(fb: &mut [u8], x: i32, y: i32, scale: f32) {
    let s = scale as i32;
    // verticals
    draw_hline(fb, x - s / 2, y, s, BLACK);
    draw_vline(fb, x, y - s / 2, s, BLACK);
    // diagonals
    let ss = (s as f32 * 0.3) as i32;
    draw_line(fb, x - ss, y - ss, x + ss, y + ss, BLACK);
    draw_line(fb, x + ss, y - ss, x - ss, y + ss, BLACK);
}

pub fn addsnow(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    for i in -2..=2 {
        snowflake(fb, x + i * 45, y + (scale * 2.0) as i32, scale);
    }
}

pub fn lightning(fb: &mut [u8], x: i32, y: i32, scale: f32, color: u8) {
    let h = 80; // total height
    let w = 45; // total width
    let dh = 20; // height of "middle" segment
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
        lightning(fb, x + i * 45, y, 1.0, BLACK);
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

pub fn addfog(fb: &mut [u8], x: i32, mut y: i32, scale: f32, mut linesize: u32, size: IconSize) {
    if size == IconSize::SMALL {
        y -= 10;
        linesize = 1u32;
    }
    for i in 0..6 {
        fill_rect(
            fb,
            x - (scale * 3.0) as i32,
            y + (scale * 1.5) as i32,
            (scale * 6.0) as i32,
            linesize as i32,
            BLACK,
        );
        fill_rect(
            fb,
            x - (scale * 3.0) as i32,
            y + (scale * 2.0) as i32,
            (scale * 6.0) as i32,
            linesize as i32,
            BLACK,
        );
        fill_rect(
            fb,
            x - (scale * 3.0) as i32,
            y + (scale * 2.5) as i32,
            (scale * 6.0) as i32,
            linesize as i32,
            BLACK,
        );
    }
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
    let mut linesize = 3;
    let mut offset = 5;

    let scale: f32;

    match size {
        IconSize::LARGE => {
            scale = LARGE as f32;
            offset = 10;
        }
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1;
        }
    }
    addcloud(fb, x, y + offset, scale, linesize);
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
    let linesize;
    match size {
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3;
        }
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1;
        }
    }

    addcloud(fb, x, y, scale, linesize);
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32,
        scale,
        size,
    );
    addcloud(fb, x, y, scale, linesize);
}

pub fn cloudy(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    let offset;
    match size {
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3;
            offset = 10;
        }
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1;
            offset = 0;
        }
    }

    addcloud(fb, x, y, scale, linesize); // Main cloud
    if size == IconSize::LARGE {
        addcloud(fb, x + 30, y, 8.0, linesize); // Cloud top right
        addcloud(fb, x - 40, y - 80, 10.0, linesize); // Cloud top left
    }
}

pub fn rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3;
        }
    }
    addcloud(fb, x, y, scale, linesize);
    addrain(fb, x, y, scale, size);
}

pub fn expect_rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1.0;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3.0;
        }
    }
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32,
        scale,
        size,
    );
    addcloud(fb, x, y, scale, linesize as u32);
    addrain(fb, x, y, scale, size);
}

pub fn chance_rain(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    expect_rain(fb, x, y, size);
}

pub fn tstorms(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1.0;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3.0;
        }
    }

    addcloud(fb, x, y, scale, linesize as u32);
    addtstorm(fb, x, y, scale);
}

pub fn snow(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1.0;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3.0;
        }
    }
    addcloud(fb, x, y, scale, linesize as u32);
    addsnow(fb, x, y, scale, size);
}

pub fn fog(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1.0;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3.0;
        }
    }
    addcloud(fb, x, y, scale, linesize as u32);
    addfog(fb, x, y - 5, scale, linesize as u32, size);
}

pub fn haze(fb: &mut [u8], x: i32, y: i32, size: IconSize) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = SMALL as f32;
            linesize = 1.0;
        }
        IconSize::LARGE => {
            scale = LARGE as f32;
            linesize = 3.0;
        }
    }
    addsun(fb, x, y - 5, scale * 1.4, size);
    addfog(fb, x, y - 5, scale * 1.4, linesize as u32, size);
}
