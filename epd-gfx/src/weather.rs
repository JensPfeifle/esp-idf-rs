use crate::font::TrueTypeText;
use crate::openmeteo::WMOCode;
use crate::weather_icons::*;

use anyhow::Result;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;

const SCREEN_W: i32 = 540;
//const SCREEN_H: i32 = 960;

pub fn draw_todays_weather<D>(
    y: i32,
    morning: (&WMOCode, &f32),
    midday: (&WMOCode, &f32),
    evening: (&WMOCode, &f32),
    night: (&WMOCode, &f32),
    display: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    let dx = SCREEN_W / 4;
    let icon_y = y + 100;
    let temp_y = y + 150;
    let icon_scale = 100;
    let font_size = 36.0;
    let mut x = dx / 2;
    {
        TrueTypeText::centered(Point::new(x, y), "Morning".into(), font_size).draw(display)?;
        draw_weather_icon(morning.0, Point::new(x, icon_y), icon_scale, display)?;
        let temp = morning.1;
        TrueTypeText::centered(Point::new(x, temp_y), format!("{temp:.0} °C"), 36.0)
            .draw(display)?;
    }
    x += dx;
    {
        TrueTypeText::centered(Point::new(x, y), "Noon".into(), font_size).draw(display)?;
        draw_weather_icon(midday.0, Point::new(x, icon_y), icon_scale, display)?;
        let temp = midday.1;
        TrueTypeText::centered(Point::new(x, temp_y), format!("{temp:.0} °C"), 36.0)
            .draw(display)?;
    }
    x += dx;
    {
        TrueTypeText::centered(Point::new(x, y), "Evening".into(), font_size).draw(display)?;
        draw_weather_icon(evening.0, Point::new(x, icon_y), icon_scale, display)?;
        let temp = evening.1;
        TrueTypeText::centered(Point::new(x, temp_y), format!("{temp:.0} °C"), 36.0)
            .draw(display)?;
    }
    x += dx;
    {
        TrueTypeText::centered(Point::new(x, y), "Night".into(), font_size).draw(display)?;
        draw_weather_icon(night.0, Point::new(x, icon_y), icon_scale, display)?;
        let temp = night.1;
        TrueTypeText::centered(Point::new(x, temp_y), format!("{temp:.0} °C"), 36.0)
            .draw(display)?;
    }
    Ok(())
}

fn draw_weather_icon<D>(
    weathercode: &WMOCode,
    pos: Point,
    scale: u32,
    display: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    // Map weather code to icon.
    let icon = match weathercode {
        WMOCode::MainlyClear | WMOCode::ClearSky => Some(Icons::ClearDay),
        WMOCode::PartyCloudy => Some(Icons::PartlyCloudyDay),
        WMOCode::Overcast => Some(Icons::Cloudy),
        WMOCode::Fog => Some(Icons::Fog),
        WMOCode::HeavyRain
        | WMOCode::LightRain
        | WMOCode::LightRainShowers
        | WMOCode::HeavyRainShowers
        | WMOCode::LightDrizzle
        | WMOCode::ModerateDrizzle
        | WMOCode::DenseDrizzle
        | WMOCode::LightFreezingRain
        | WMOCode::HeavyFreezingRain
        | WMOCode::ModerateRain => Some(Icons::Rain),
        WMOCode::LightSnow
        | WMOCode::HeavySnow
        | WMOCode::SnowGrains
        | WMOCode::ModerateSnow
        | WMOCode::LightSnowShowers
        | WMOCode::HeavySnowShowers => Some(Icons::Snow),
        WMOCode::Thunderstorm
        | WMOCode::ThunderstormWithHeavyHail
        | WMOCode::ThunderstormWithLightHail => Some(Icons::Thunderstorm),
        _ => None,
    };

    // Draw the icon, falling back to a text representation if needed.
    match icon {
        Some(i) => match i {
            Icons::ClearDay => ClearDay::new().translate(pos).scale(scale).draw(display)?,
            Icons::ClearNight => ClearNight::new()
                .translate(pos)
                .scale(scale)
                .draw(display)?,
            Icons::PartlyCloudyDay => PartlyCloudyDay::new()
                .translate(pos)
                .scale(scale)
                .draw(display)?,
            Icons::PartlyCloudyNight => PartlyCloudyNight::new()
                .translate(pos)
                .scale(scale)
                .draw(display)?,
            Icons::Cloudy => Cloudy::new().translate(pos).scale(scale).draw(display)?,
            Icons::Fog => Fog::new().translate(pos).scale(scale).draw(display)?,
            Icons::Rain => Rain::new().translate(pos).scale(scale).draw(display)?,
            Icons::Snow => Snow::new().translate(pos).scale(scale).draw(display)?,
            Icons::Thunderstorm => Thunderstorm::new()
                .translate(pos)
                .scale(scale)
                .draw(display)?,
        },
        None => {
            TrueTypeText::new(pos, weathercode.to_string(), 30.0).draw(display)?;
        }
    };
    Ok(())
}

pub fn draw_current_weather<D>(
    temperature: f32,
    winddirection: u32,
    windspeed: f32,
    weathercode: &WMOCode,
    display: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    // Current weather icon
    draw_weather_icon(&weathercode, Point { x: 125, y: 170 }, 200, display)?;

    TrueTypeText::new(
        Point { x: 250, y: 100 },
        format!("{temperature:.0} °C"),
        50.0,
    )
    .draw(display)?;

    TrueTypeText::new(
        Point { x: 250, y: 150 },
        format!("{windspeed:.0} kts @ {winddirection} °"),
        50.0,
    )
    .draw(display)?;

    Ok(())
}

pub fn draw_icon_test_page<D>(scale: u32, display: &mut D) -> Result<(), core::convert::Infallible>
where
    D: DrawTarget<Color = Gray4, Error = core::convert::Infallible>,
{
    let x1 = 120;
    let x2 = 400;

    let dy = 180;
    let mut y = 100;

    ClearDay::new()
        .translate(Point::new(x1, y))
        .scale(scale)
        .draw(display)?;

    ClearNight::new()
        .translate(Point::new(x2, y))
        .scale(scale)
        .draw(display)?;

    y += dy;
    PartlyCloudyDay::new()
        .translate(Point::new(x1, y))
        .scale(scale)
        .draw(display)?;
    PartlyCloudyNight::new()
        .translate(Point::new(x2, y))
        .scale(scale)
        .draw(display)?;

    y += dy;
    // ?

    Rain::new()
        .translate(Point::new(x2, y))
        .scale(scale)
        .draw(display)?;

    y += dy;
    Snow::new()
        .translate(Point::new(x1, y))
        .scale(scale)
        .draw(display)?;
    Thunderstorm::new()
        .translate(Point::new(x2, y))
        .scale(scale)
        .draw(display)?;

    y += dy;
    Fog::new()
        .translate(Point::new(x1, y))
        .scale(scale)
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
