use anyhow::Result;
use brtsky;
use embedded_graphics::{
    pixelcolor::Gray4,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_svc::http::client::Response;
use embedded_svc::http::Status;
use epd_gfx;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use esp_idf_sys::{vTaskDelay, TickType_t};
use serde_json;
use std::sync::Arc;
pub mod epd;
pub mod epd_highlevel;
pub mod wifi;

unsafe fn delay() {
    //https://github.com/espressif/esp-idf/issues/1646#issuecomment-913190625
    let delay: TickType_t = 500;
    vTaskDelay(delay);
}

fn icons(fb: &mut [u8]) {
    let x1 = 120;
    let x2 = 400;

    let dy = 180;
    let mut y = 100;

    epd_gfx::drawing::draw_vline(fb, x1, 0, 960, 0x8);
    epd_gfx::drawing::draw_vline(fb, x2, 0, 960, 0x8);

    epd_gfx::drawing::draw_hline(fb, 0, y, 540, 0x8);
    epd_gfx::icons::sunny(fb, x1, y, epd_gfx::icons::IconSize::LARGE);
    epd_gfx::icons::mostly_sunny(fb, x2, y, epd_gfx::icons::IconSize::LARGE);

    y += dy;
    epd_gfx::drawing::draw_hline(fb, 0, y, 540, 0x8);
    epd_gfx::icons::mostly_cloudy(fb, x1, y, epd_gfx::icons::IconSize::LARGE);
    epd_gfx::icons::cloudy(fb, x2, y, epd_gfx::icons::IconSize::LARGE);
    y += dy;
    epd_gfx::drawing::draw_hline(fb, 0, y, 540, 0x8);
    epd_gfx::icons::rain(fb, x1, y, epd_gfx::icons::IconSize::LARGE);
    epd_gfx::icons::expect_rain(fb, x2, y, epd_gfx::icons::IconSize::LARGE);
    y += dy;
    epd_gfx::drawing::draw_hline(fb, 0, y, 540, 0x8);
    epd_gfx::icons::tstorms(fb, x2, y, epd_gfx::icons::IconSize::LARGE);
    epd_gfx::icons::snow(fb, x1, y, epd_gfx::icons::IconSize::LARGE);
    y += dy;
    epd_gfx::drawing::draw_hline(fb, 0, y, 540, 0x8);
    epd_gfx::icons::fog(fb, x1, y, epd_gfx::icons::IconSize::LARGE);
    epd_gfx::icons::haze(fb, x2, y, epd_gfx::icons::IconSize::LARGE);
}

fn main() -> Result<()> {
    fetch()?;
    draw_screen()?;
    Ok(())
}

fn fetch() -> Result<()> {
    println!("initializing...");

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    let mut wifi = wifi::wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    let mut client = wifi::WeatherApi::new()?;
    let response = client.get()?;
    let code = response.status();
    println!("status code: {code}");

    let bytes: Result<Vec<_>, _> =
        embedded_svc::io::Bytes::<_, 64>::new(response.reader()).collect();
    let body = bytes?;
    let data: brtsky::Response = serde_json::from_slice(&body).unwrap();
    println!("data: {data:?}");
    Ok(())
}

fn draw_screen() -> Result<()> {
    let mut epd = epd::Epd::new();
    epd.clear();

    println!("drawing...");
    let mut fb = epd.get_mut_buffer();
    //epd_gfx::set_all(&mut fb, 0xFF);
    //icons(&mut fb);
    epd_gfx::font::draw_text(&mut fb, 0, 0, "Hello from RustType!", 32);

    Circle::new(Point::new(50, 50), 50)
        .into_styled(PrimitiveStyle::with_stroke(Gray4::BLACK, 1))
        .draw(&mut epd)?;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Gray4::new(0x4))
        .stroke_width(3)
        .fill_color(Gray4::new(0x8))
        .build();

    Circle::new(Point::new(200, 500), 100)
        .into_styled(style)
        .draw(&mut epd)?;

    //self.icons();

    epd.update_screen(25i32);
    Ok(())
}

#[no_mangle]
extern "C" fn app_main() {
    match main() {
        Ok(_) => {}
        Err(err) => {
            println!("Unhandled error in main:");
            println!("{err:?}");
        }
    }

    println!("looping forever...");
    loop {
        unsafe { delay() };
    }
}
