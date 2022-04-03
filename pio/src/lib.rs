use anyhow::{bail, Context, Result};
use embedded_svc::http::client::Response;
use embedded_svc::http::Status;
use epd_gfx;
use epd_gfx::openmeteo::responses::OpenMeteoCurrentWeather;
use epd_gfx::openmeteo::{self, OpenMeteoData, OpenMeteoError};
use epd_gfx::openmeteo::{Location, OpenMeteoConfig};
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::EspWifi;
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
            println!("Unhandled error in main: {}", err);
        }
    }

    println!("Reached end of main - looping forever...");
    loop {
        unsafe { delay() };
    }
}

fn main() -> Result<()> {
    // Bind the log crate to the ESP Logging facilities
    // -> crashes :(
    //esp_idf_svc::log::EspLogger::initialize_default();

    let wifi = wifi_init()?;
    let weather = fetch_weather()?;
    if let Some(current) = weather.current_weather {
    draw_screen(current)?;
    }
    Ok(())
}

fn wifi_init() -> Result<Box<EspWifi>> {
    println!("INITIALIZING WIFI...");

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    let wifi = wifi::wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;
    println!("WIFI INITIALIZATION COMPLETE");
    Ok(wifi)
}

fn fetch_weather() -> Result<OpenMeteoData> {
    println!("FETCHING DATA...");

    let mut client = wifi::WeatherApi::new(OpenMeteoConfig::new(Location {
        lat: 48.93,
        lon: 8.4,
    }))?;
    let response = client.get()?;
    let code = response.status();
    println!("status code: {code}");
    let bytes: Result<Vec<_>, _> =
        embedded_svc::io::Bytes::<_, 64>::new(response.reader()).collect();
    let body = bytes?;
    if let Ok(data) = serde_json::from_slice::<openmeteo::OpenMeteoData>(&body) {
        println!("data: {data:?}");
        return Ok(data);
    }
    if let Ok(err) = serde_json::from_slice::<openmeteo::OpenMeteoError>(&body) {
        bail!("400 error: {:?}", err);
    }
    bail!("Unable to decode response! {:?}", body);
}

fn draw_screen(weather: OpenMeteoCurrentWeather) -> Result<()> {
    println!("DRAWING...");
    let mut epd = epd::Epd::new();
    epd.clear();

    epd_gfx::draw_header("21:24", "03.04.2022", &mut epd);
    epd_gfx::draw_current_weather(&weather.weathercode, weather.temperature, &mut epd);

    epd.update_screen(25i32);
    println!("DRAW COMPLETE");
    Ok(())
}
