use crate::weather_icons::drawables::{Cloud, Lightning, Moon, Raindrop, Snowflake, Sun};
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::Drawable;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Icons {
    ClearDay,
    ClearNight,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Cloudy,
    Fog,
    Rain,
    Snow,
    Thunderstorm,
}

pub trait Scale {
    fn scale(&self, scale: u32) -> Self;
    fn scale_mut(&mut self, scale: u32) -> &mut Self;
}

/// Add a `new` method to initialize struct with pos and scale fields.
macro_rules! new {
    ($type:ident) => {
        impl $type {
            pub fn new() -> Self {
                Self {
                    pos: Point { x: 0, y: 0 },
                    scale: 100,
                }
            }
        }
    };
}

/// Derive Scale trait for a struct with pos and scale fields.
macro_rules! scale {
    ($type:ident) => {
        impl Scale for $type {
            fn scale(&self, scale: u32) -> Self {
                Self {
                    pos: self.pos,
                    scale,
                }
            }

            fn scale_mut(&mut self, scale: u32) -> &mut Self {
                self.scale = scale;
                self
            }
        }
    };
}

/// Derive Transform trait for a struct with pos and scale fields.
macro_rules! transform {
    ($type:ident) => {
        impl Transform for $type {
            fn translate(&self, pos: Point) -> Self {
                Self {
                    pos,
                    scale: self.scale,
                }
            }

            fn translate_mut(&mut self, pos: Point) -> &mut Self {
                self.pos = pos;
                self
            }
        }
    };
}

pub struct ClearDay {
    pub pos: Point,
    pub scale: u32,
}

new!(ClearDay);
transform!(ClearDay);
scale!(ClearDay);

impl Drawable for ClearDay {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Sun::new(self.pos, self.scale).draw(target)?;
        Ok(())
    }
}

pub struct ClearNight {
    pub pos: Point,
    pub scale: u32,
}

new!(ClearNight);
transform!(ClearNight);
scale!(ClearNight);

impl Drawable for ClearNight {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Moon::new(self.pos, self.scale / 3 * 2).draw(target)?;
        Ok(())
    }
}

pub struct PartlyCloudyDay {
    pub pos: Point,
    pub scale: u32,
}

new!(PartlyCloudyDay);
transform!(PartlyCloudyDay);
scale!(PartlyCloudyDay);

impl Drawable for PartlyCloudyDay {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, self.scale).draw(target)?;
        let sun_x = self.pos.x - self.scale as i32 / 3;
        let sun_y = self.pos.y - self.scale as i32 / 3;
        let sun_pos = Point::new(sun_x, sun_y);
        Sun::new(sun_pos, self.scale * 2 / 3).draw(target)?;

        Ok(())
    }
}

pub struct PartlyCloudyNight {
    pub pos: Point,
    pub scale: u32,
}

new!(PartlyCloudyNight);
transform!(PartlyCloudyNight);
scale!(PartlyCloudyNight);

impl Drawable for PartlyCloudyNight {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let moon_x = self.pos.x - self.scale as i32 / 3;
        let moon_y = self.pos.y - self.scale as i32 / 3;
        let moon_pos = Point::new(moon_x, moon_y);
        Moon::new(moon_pos, self.scale / 3).draw(target)?;

        Cloud::new(self.pos, self.scale).draw(target)?;

        Ok(())
    }
}

pub struct Cloudy {
    pub pos: Point,
    pub scale: u32,
}
new!(Cloudy);
transform!(Cloudy);
scale!(Cloudy);

impl Drawable for Cloudy {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, self.scale).draw(target)?;

        Ok(())
    }
}

pub struct Fog {
    pub pos: Point,
    pub scale: u32,
}

new!(Fog);
transform!(Fog);
scale!(Fog);

impl Drawable for Fog {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let x_step = self.scale as i32 / 2;
        let y_step = self.scale as i32 / 5;
        let stroke = self.scale / 10;
        for dy in -1..=1 {
            let y = self.pos.y + y_step * dy;
            let left = Point::new(self.pos.x - x_step, y);
            let right = Point::new(self.pos.x + x_step, y);
            Line::new(left, right)
                .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x9), stroke))
                .draw(target)?;
        }
        Ok(())
    }
}

pub struct Rain {
    pub pos: Point,
    pub scale: u32,
}
new!(Rain);
transform!(Rain);
scale!(Rain);

impl Drawable for Rain {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, self.scale).draw(target)?;

        let x_step = self.scale as i32 / 6;
        let y_offset = self.scale as i32 / 4;
        let drop_size = self.scale / 15;
        for dx in -2..=2 {
            let pos = Point::new(self.pos.x - x_step * dx, self.pos.y + y_offset);
            Raindrop::new(pos, drop_size).draw(target)?;
        }

        Ok(())
    }
}

pub struct Snow {
    pub pos: Point,
    pub scale: u32,
}

new!(Snow);
transform!(Snow);
scale!(Snow);

impl Drawable for Snow {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, self.scale).draw(target)?;

        let x_step = self.scale as i32 / 6;
        let y_step = self.scale as i32 / 4;
        let flake_size = self.scale / 15;
        for dx in -2..=2 {
            let pos = Point::new(self.pos.x - x_step * dx, self.pos.y + y_step);
            Snowflake::new(pos, flake_size).draw(target)?;
        }

        Ok(())
    }
}

pub struct Thunderstorm {
    pub pos: Point,
    pub scale: u32,
}
new!(Thunderstorm);
transform!(Thunderstorm);
scale!(Thunderstorm);

impl Drawable for Thunderstorm {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, self.scale).draw(target)?;

        let x_step = self.scale as i32 / 4;
        let y_step = self.scale as i32 / 15 * 4;
        let bolt_size = self.scale / 6;
        for dx in -1..=1 {
            let pos = Point::new(self.pos.x - x_step * dx, self.pos.y + y_step);
            Lightning::new(pos, bolt_size).draw(target)?;
        }

        Ok(())
    }
}
