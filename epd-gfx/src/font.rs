use embedded_graphics::{pixelcolor::Gray4, prelude::*};

use rusttype::{point, Font, Scale};

struct TrueTypeText {
    pos: Point,
    size: f32,
    text: Box<String>,
}

impl TrueTypeText {
    pub fn new(pos: Point, text: String, size: f32) -> Self {
        Self {
            pos,
            size,
            text: Box::new(text),
        }
    }
}

impl Drawable for TrueTypeText {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // Load the font
        let font_data = include_bytes!("../fonts/Karla-Medium.ttf");
        // This only succeeds if collection consists of one font
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        // The font size to use
        let scale = Scale::uniform(self.size);

        let v_metrics = font.v_metrics(scale);

        // layout the glyphs in a line
        let glyphs: Vec<_> = font
            .layout(&self.text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let x_pos = self.pos.x;
        let y_pos = self.pos.y;
        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    Pixel(
                        Point::new(
                            // Offset the position by the glyph bounding box
                            (x_pos + x as i32 + bounding_box.min.x) as i32,
                            (y_pos + y as i32 + bounding_box.min.y) as i32,
                        ),
                        // Turn the coverage into a grayscale value
                        Gray4::new(((1.0 - v) * 15.0) as u8),
                    )
                    .draw(target)
                    .map_err(|_| ()) // DrawTarget::Error doesn't implement Debug?
                    .expect("Error drawing text to target!");
                });
            }
        }
        Ok(())
    }
}
