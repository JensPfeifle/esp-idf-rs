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

fn icons(display: &mut epd::Epd) -> Result<()> {
    let x1 = 120;
    let x2 = 400;

    let dy = 180;
    let mut y = 100;

    use epd_gfx::icons::*;

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
    icons(&mut epd);
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
