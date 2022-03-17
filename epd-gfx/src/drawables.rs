use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, Rectangle};
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::Drawable;

pub struct Cloud<C: PixelColor> {
    pos: Point,
    fg_color: C,
    bg_color: C,
    size: u32,
}

impl Cloud<Gray4> {
    pub fn new(pos: Point, size: u32) -> Self {
        Self {
            pos,
            fg_color: Gray4::BLACK,
            bg_color: Gray4::WHITE,
            size,
        }
    }
}

impl<C> Drawable for Cloud<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let width = self.size as i32;
        let height = self.size as i32;

        let stroke = (self.size / 30).max(1);

        // fill shape, for outline
        {
            // outer circles: cl, cr
            {
                let r = height / 6;
                let pos_cl = Point {
                    x: self.pos.x - width / 2 + r,
                    y: self.pos.y,
                };
                let pos_cr = Point {
                    x: self.pos.x + width / 2 - r,
                    y: self.pos.y,
                };
                let r = r as u32;
                Circle::with_center(pos_cl, 2 * r)
                    .into_styled(PrimitiveStyle::with_fill(self.fg_color))
                    .draw(target)?;
                Circle::with_center(pos_cr, 2 * r)
                    .into_styled(PrimitiveStyle::with_fill(self.fg_color))
                    .draw(target)?;

                // rectangle
                {
                    Rectangle::with_center(
                        self.pos,
                        Size {
                            width: self.size - (2 * r),
                            height: 2 * r,
                        },
                    )
                    .into_styled(PrimitiveStyle::with_fill(self.fg_color))
                    .draw(target)?;
                }
            }

            // top left circle
            {
                let r = (height / 5) as u32;
                let p = Point {
                    x: self.pos.x - height / 6,
                    y: self.pos.y - height / 6,
                };
                Circle::with_center(p, 2 * r)
                    .into_styled(PrimitiveStyle::with_fill(self.fg_color))
                    .draw(target)?;
            }

            // top right circle
            {
                let r = (height / 4) as u32;
                let p = Point {
                    x: self.pos.x + height / 7,
                    y: self.pos.y - height / 5,
                };
                Circle::with_center(p, 2 * r)
                    .into_styled(PrimitiveStyle::with_fill(self.fg_color))
                    .draw(target)?;
            }
        }

        // fill 'empty' space inside
        {
            // outer circles: cl, cr
            {
                let r = height / 6;
                let pos_cl = Point {
                    x: self.pos.x - width / 2 + r,
                    y: self.pos.y,
                };
                let pos_cr = Point {
                    x: self.pos.x + width / 2 - r,
                    y: self.pos.y,
                };
                Circle::with_center(pos_cl, 2 * (r as u32 - stroke))
                    .into_styled(PrimitiveStyle::with_fill(self.bg_color))
                    .draw(target)?;
                Circle::with_center(pos_cr, 2 * (r as u32 - stroke))
                    .into_styled(PrimitiveStyle::with_fill(self.bg_color))
                    .draw(target)?;

                // rectangle
                {
                    Rectangle::with_center(
                        self.pos,
                        Size {
                            width: self.size - (2 * r as u32),
                            height: 2 * (r as u32 - stroke),
                        },
                    )
                    .into_styled(PrimitiveStyle::with_fill(self.bg_color))
                    .draw(target)?;
                }
            }

            // top left circle
            {
                let r = (height / 5) - stroke as i32;
                let p = Point {
                    x: self.pos.x - height / 6,
                    y: self.pos.y - height / 6,
                };
                Circle::with_center(p, 2 * r as u32)
                    .into_styled(PrimitiveStyle::with_fill(self.bg_color))
                    .draw(target)?;
            }

            // top right circle
            {
                let r = (height / 4) as u32 - stroke;
                let p = Point {
                    x: self.pos.x + height / 7,
                    y: self.pos.y - height / 5,
                };
                Circle::with_center(p, 2 * r as u32)
                    .into_styled(PrimitiveStyle::with_fill(self.bg_color))
                    .draw(target)?;
            }
        }

        Ok(())
    }
}

pub struct Sun<C: PixelColor> {
    pos: Point,
    fg_color: C,
    bg_color: C,
    size: Size,
}

impl Sun<Gray4> {
    pub fn new(pos: Point, size: u32) -> Self {
        Self {
            pos,
            fg_color: Gray4::BLACK,
            bg_color: Gray4::WHITE,
            size: Size {
                width: size,
                height: size,
            },
        }
    }
}

impl<C> Drawable for Sun<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let width = self.size.width as i32;
        let height = self.size.height as i32;

        // Horizontal rays
        Line::new(
            Point::new(self.pos.x - width / 2, self.pos.y),
            Point::new(self.pos.x + width / 2, self.pos.y),
        )
        .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 2))
        .draw(target)?;

        // Vertical rays
        Line::new(
            Point::new(self.pos.x, self.pos.y - height / 2),
            Point::new(self.pos.x, self.pos.y + height / 2),
        )
        .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 2))
        .draw(target)?;

        // Diagonal rays
        let ray = ((width / 2) as f32 * 0.65) as i32;
        Line::new(
            Point::new(self.pos.x - ray, self.pos.y - ray),
            Point::new(self.pos.x + ray, self.pos.y + ray),
        )
        .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 2))
        .draw(target)?;
        Line::new(
            Point::new(self.pos.x - ray, self.pos.y + ray),
            Point::new(self.pos.x + ray, self.pos.y - ray),
        )
        .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 2))
        .draw(target)?;

        let sun_diameter = (width / 2) as u32;
        let ray_distance = (sun_diameter as f32 * 1.2) as u32;
        Circle::with_center(self.pos, ray_distance)
            .into_styled(PrimitiveStyle::with_fill(self.bg_color))
            .draw(target)?;

        Circle::with_center(self.pos, sun_diameter)
            .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 3))
            .draw(target)?;

        Ok(())
    }
}
