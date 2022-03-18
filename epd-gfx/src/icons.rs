use crate::drawables::{Cloud, Lightning, Moon, Raindrop, Snowflake, Sun};
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::Drawable;
pub struct ClearDay {
    pub pos: Point,
}

impl Drawable for ClearDay {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Sun::new(self.pos, 150).draw(target)?;
        Ok(())
    }
}

pub struct ClearNight {
    pub pos: Point,
}

impl Drawable for ClearNight {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Moon::new(self.pos, 150).draw(target)?;
        Ok(())
    }
}

pub struct PartlyCloudyDay {
    pub pos: Point,
}

impl Drawable for PartlyCloudyDay {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, 150).draw(target)?;

        let sun_pos = Point::new(self.pos.x - 45, self.pos.y - 40);
        Sun::new(sun_pos, 100).draw(target)?;

        Ok(())
    }
}

pub struct PartlyCloudyNight {
    pub pos: Point,
}

impl Drawable for PartlyCloudyNight {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let moon_pos = Point::new(self.pos.x - 50, self.pos.y - 45);
        Moon::new(moon_pos, 50).draw(target)?;

        Cloud::new(self.pos, 150).draw(target)?;

        Ok(())
    }
}

pub struct Cloudy {
    pub pos: Point,
}

impl Drawable for Cloudy {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, 150).draw(target)?;

        Ok(())
    }
}

pub struct Fog {
    pub pos: Point,
}

impl Drawable for Fog {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for dy in -1..=1 {
            let left = Point::new(self.pos.x - 75, self.pos.y + 30 * dy);
            let right = Point::new(self.pos.x + 75, self.pos.y + 30 * dy);
            Line::new(left, right)
                .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x9), 15))
                .draw(target)?;
        }
        Ok(())
    }
}

pub struct Rain {
    pub pos: Point,
}

impl Drawable for Rain {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, 150).draw(target)?;
        for dx in -2..=2 {
            let pos = Point::new(self.pos.x - 25 * dx, self.pos.y + 40);
            Raindrop::new(pos, 10).draw(target)?;
        }

        Ok(())
    }
}

pub struct Snow {
    pub pos: Point,
}

impl Drawable for Snow {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, 150).draw(target)?;
        for dx in -2..=2 {
            let pos = Point::new(self.pos.x - 25 * dx, self.pos.y + 40);
            Snowflake::new(pos, 15).draw(target)?;
        }

        Ok(())
    }
}

pub struct Thunderstorm {
    pub pos: Point,
}

impl Drawable for Thunderstorm {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Cloud::new(self.pos, 150).draw(target)?;
        for dx in -1..=1 {
            let pos = Point::new(self.pos.x - 45 * dx, self.pos.y + 40);
            Lightning::new(pos, 25).draw(target)?;
        }

        Ok(())
    }
}

pub struct Wind {
    pub pos: Point,
}

impl Drawable for Wind {
    type Color = Gray4;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for dy in -4..=1 {
            let start = Point::new(self.pos.x - 100, self.pos.y + 10 * dy);
            let end = Point::new(self.pos.x, self.pos.y + 10 * dy);
            Line::new(start, end)
                .into_styled(PrimitiveStyle::with_stroke(Gray4::BLACK, 3))
                .draw(target)?;
        }
        Cloud::new(self.pos, 150).draw(target)?;

        Ok(())
    }
}
