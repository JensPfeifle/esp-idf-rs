pub mod font;
pub mod openmeteo;
pub mod weather_icons;

use crate::font::TrueTypeText;
use crate::openmeteo::WMOCode;
use crate::weather_icons::*;

use anyhow::Result;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Line;
use embedded_graphics::primitives::PrimitiveStyle;

/// Draw the header / "top bar" with an underline.
pub fn draw_header<D>(
    time: &str,
    date: &str,
    display: &mut D,
) -> Result<(), core::convert::Infallible>
where
    D: DrawTarget<Color = Gray4, Error = core::convert::Infallible>,
{
    TrueTypeText::new(Point { x: 0, y: 0 }, time.into(), 24.0).draw(display)?;
    TrueTypeText::new(Point { x: 420, y: 0 }, date.into(), 24.0).draw(display)?;
    Line::new(Point { x: 0, y: 30 }, Point { x: 540, y: 30 })
        .into_styled(PrimitiveStyle::with_stroke(Gray4::BLACK, 2))
        .draw(display)?;
    Ok(())
}

pub fn draw_current_weather<D>(code: &WMOCode, temperature: f32, display: &mut D) -> Result<()>
where
    D: DrawTarget<Color = Gray4, Error = core::convert::Infallible>,
{
    // Current weather icon
    let icon_pos = Point { x: 120, y: 150 };

    match code {
        WMOCode::MainlyClear | WMOCode::ClearSky => ClearDay { pos: icon_pos }.draw(display)?,
        WMOCode::PartyCloudy => PartlyCloudyDay { pos: icon_pos }.draw(display)?,
        WMOCode::Overcast => icons::Cloudy { pos: icon_pos }.draw(display)?,
        WMOCode::Fog => Fog { pos: icon_pos }.draw(display)?,
        //Wind => Wind { pos: icon_pos }.draw(display)?,
        WMOCode::HeavyRain
        | WMOCode::LightRain
        | WMOCode::LightDrizzle
        | WMOCode::ModerateDrizzle
        | WMOCode::DenseDrizzle
        | WMOCode::LightFreezingRain
        | WMOCode::HeavyFreezingRain
        | WMOCode::ModerateRain => Rain { pos: icon_pos }.draw(display)?,
        WMOCode::LightSnow
        | WMOCode::HeavySnow
        | WMOCode::SnowGrains
        | WMOCode::ModerateSnow
        | WMOCode::LightSnowShowers
        | WMOCode::HeavySnowShowers => Snow { pos: icon_pos }.draw(display)?,
        WMOCode::Thunderstorm
        | WMOCode::ThunderstormWithHeavyHail
        | WMOCode::ThunderstormWithLightHail => {
            icons::Thunderstorm { pos: icon_pos }.draw(display)?
        }
        _ => {
            // Fall back to text.
            TrueTypeText::new(icon_pos, code.to_string(), 30.0).draw(display)?;
        }
    }

    TrueTypeText::new(Point { x: 400, y: 50 }, format!("{temperature} °C"), 50.0).draw(display)?;

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
    let height = 960i32;
    let width = 540i32;

    let style = PrimitiveStyle::with_stroke(Gray4::new(luma), 1);

    for y in (y_step..height).step_by(y_step as usize) {
        Line::new(Point { x: 0, y }, Point { x: width, y })
            .into_styled(style)
            .draw(display)?;
    }

    for x in (x_step..width).step_by(x_step as usize) {
        Line::new(Point { x, y: 0 }, Point { x, y: height })
            .into_styled(style)
            .draw(display)?;
    }

    Ok(())
}

pub fn draw_icon_test_page<D>(display: &mut D) -> Result<(), core::convert::Infallible>
where
    D: DrawTarget<Color = Gray4, Error = core::convert::Infallible>,
{
    let x1 = 120;
    let x2 = 400;

    let dy = 180;
    let mut y = 100;

    ClearDay {
        pos: Point::new(x1, y),
    }
    .draw(display)?;

    ClearNight {
        pos: Point::new(x2, y),
    }
    .draw(display)?;

    y += dy;
    PartlyCloudyDay {
        pos: Point::new(x1, y),
    }
    .draw(display)?;
    PartlyCloudyNight {
        pos: Point::new(x2, y),
    }
    .draw(display)?;

    y += dy;
    Wind {
        pos: Point::new(x1, y),
    }
    .draw(display)?;
    Rain {
        pos: Point::new(x2, y),
    }
    .draw(display)?;

    y += dy;
    Snow {
        pos: Point::new(x1, y),
    }
    .draw(display)?;
    Thunderstorm {
        pos: Point::new(x2, y),
    }
    .draw(display)?;

    y += dy;
    Fog {
        pos: Point::new(x1, y),
    }
    .draw(display)?;
    Ok(())
}

pub fn deg_to_direction(winddirection: f32) -> String {
    if winddirection >= 348.75 || winddirection < 11.25 {
        return String::from("N");
    }
    if winddirection >= 11.25 && winddirection < 33.75 {
        return String::from("NNE");
    }
    if winddirection >= 33.75 && winddirection < 56.25 {
        return String::from("NE");
    }
    if winddirection >= 56.25 && winddirection < 78.75 {
        return String::from("ENE");
    }
    if winddirection >= 78.75 && winddirection < 101.25 {
        return String::from("E");
    }
    if winddirection >= 101.25 && winddirection < 123.75 {
        return String::from("ESE");
    }
    if winddirection >= 123.75 && winddirection < 146.25 {
        return String::from("SE");
    }
    if winddirection >= 146.25 && winddirection < 168.75 {
        return String::from("SSE");
    }
    if winddirection >= 168.75 && winddirection < 191.25 {
        return String::from("S");
    }
    if winddirection >= 191.25 && winddirection < 213.75 {
        return String::from("SSW");
    }
    if winddirection >= 213.75 && winddirection < 236.25 {
        return String::from("SW");
    }
    if winddirection >= 236.25 && winddirection < 258.75 {
        return String::from("WSW");
    }
    if winddirection >= 258.75 && winddirection < 281.25 {
        return String::from("W");
    }
    if winddirection >= 281.25 && winddirection < 303.75 {
        return String::from("WNW");
    }
    if winddirection >= 303.75 && winddirection < 326.25 {
        return String::from("NW");
    }
    if winddirection >= 326.25 && winddirection < 348.75 {
        return String::from("NNW");
    }
    return String::from("?");
}
