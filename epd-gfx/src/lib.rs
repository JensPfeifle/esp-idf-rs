pub mod font;
pub mod kvv;
pub mod openmeteo;
pub mod weather;
pub mod weather_icons;

use crate::font::TrueTypeText;
use anyhow::Result;
use bytes::Bytes;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Line;
use embedded_graphics::primitives::PrimitiveStyle;
use openmeteo::OpenMeteoData;

const SCREEN_W: i32 = 540;
const SCREEN_H: i32 = 960;

pub trait Fetch {
    fn get(url: &str) -> Result<Bytes>;
}

/// Draw the header / "top bar" with an underline.
pub fn draw_header<D>(time: &str, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    TrueTypeText::centered(Point::new(SCREEN_W / 2, 0), time.into(), 36.0).draw(display)?;
    Line::new(Point { x: 0, y: 40 }, Point { x: SCREEN_W, y: 40 })
        .into_styled(PrimitiveStyle::with_stroke(Gray4::BLACK, 2))
        .draw(display)?;
    Ok(())
}

pub fn draw_grid<D>(
    x_step: i32,
    y_step: i32,
    luma: u8,
    display: &mut D,
) -> Result<(), core::convert::Infallible>
where
    D: DrawTarget<Color = Gray4, Error = core::convert::Infallible>,
{
    let style = PrimitiveStyle::with_stroke(Gray4::new(luma), 1);

    for y in (y_step..SCREEN_H).step_by(y_step as usize) {
        Line::new(Point { x: 0, y }, Point { x: SCREEN_W, y })
            .into_styled(style)
            .draw(display)?;
    }

    for x in (x_step..SCREEN_W).step_by(x_step as usize) {
        Line::new(Point { x, y: 0 }, Point { x, y: SCREEN_H })
            .into_styled(style)
            .draw(display)?;
    }

    Ok(())
}

pub fn draw<D>(weather: &OpenMeteoData, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    DrawTarget::clear(display, Gray4::new(0xF))?;

    if let Some(ref current_weather) = weather.current_weather {
        draw_header(&current_weather.time, display)?;
    }
    draw_weather(weather, display)?;

    Ok(())
}

pub fn draw_weather<D>(data: &OpenMeteoData, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    if let Some(ref current_weather) = data.current_weather {
        draw_header(&current_weather.time, display)?;
        weather::draw_current_weather(
            current_weather.temperature,
            current_weather.winddirection,
            current_weather.windspeed,
            &current_weather.weathercode,
            display,
        )?;
    }
    if let Some(ref hourly) = data.hourly {
        let morning = (
            hourly.weathercode.iter().flatten().nth(9).unwrap(),
            hourly.apparent_temperature.iter().flatten().nth(9).unwrap(),
        );
        let midday = (
            hourly.weathercode.iter().flatten().nth(12).unwrap(),
            hourly
                .apparent_temperature
                .iter()
                .flatten()
                .nth(12)
                .unwrap(),
        );
        let evening = (
            hourly.weathercode.iter().flatten().nth(18).unwrap(),
            hourly
                .apparent_temperature
                .iter()
                .flatten()
                .nth(18)
                .unwrap(),
        );
        let night = (
            hourly.weathercode.iter().flatten().nth(24).unwrap(),
            hourly
                .apparent_temperature
                .iter()
                .flatten()
                .nth(24)
                .unwrap(),
        );
        weather::draw_todays_weather(250, morning, midday, evening, night, display)?;
    }
    Ok(())
}
