use crate::draw_pixel;
use rusttype::{point, Font, Scale};

pub fn draw_text(fb: &mut [u8], x: u32, y: u32, text: &str, size: u32) {
    // Load the font
    let font_data = include_bytes!("../fonts/Karla-Medium.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(size as f32);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();

    let x_pos = x;
    let y_pos = y;
    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                draw_pixel(
                    fb,
                    // Offset the position by the glyph bounding box
                    x_pos + x + bounding_box.min.x as u32,
                    y_pos + y + bounding_box.min.y as u32,
                    // Turn the coverage into a grayscale value
                    ((1.0 - v) * 15.0) as u8,
                )
            });
        }
    }
}
