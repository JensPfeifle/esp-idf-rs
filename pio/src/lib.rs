use anyhow::{Context, Result};
use brightsky::models::responses::CurrentWeatherResponse;
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
    let data: CurrentWeatherResponse =
        serde_json::from_slice(&body).context("Unable to decode weather data")?;
    println!("data: {data:?}");
    Ok(())
}

fn draw_screen() -> Result<()> {
    let mut epd = epd::Epd::new();
    epd.clear();

    println!("drawing...");
    let mut fb = epd.get_mut_buffer();
    epd.update_screen(25i32);
    Ok(())
}
