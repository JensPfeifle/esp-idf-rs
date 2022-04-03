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
use std::thread;
use std::time::*;
pub mod epd;
pub mod epd_highlevel;
pub mod wifi;
use core::time::Duration;
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::EspTimerService;
use esp_idf_svc::timer::*;
unsafe fn delay() {
    //https://github.com/espressif/esp-idf/issues/1646#issuecomment-913190625
    let delay: TickType_t = 500;
    vTaskDelay(delay);
}

fn test_timer() -> Result<EspTimer> {
    thread::sleep(Duration::from_secs(3));

    println!("About to schedule a periodic timer every five seconds");
    let mut periodic_timer = EspTimerService::new()?.timer(move || {
        println!("Tick from periodic timer");
    })?;

    periodic_timer.every(Duration::from_secs(60))?;

    Ok(periodic_timer)
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

    let _t = test_timer()?;
    let _wifi = wifi_init()?;

    let mut epd = epd::Epd::new();
    epd.clear();

    loop {
        let weather = fetch_weather()?;
        if let Some(current) = weather.current_weather {
            draw_screen(&mut epd, current)?;
        }
        thread::sleep(Duration::from_secs(300));
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

fn draw_screen(epd: &mut epd::Epd, weather: OpenMeteoCurrentWeather) -> Result<()> {
    println!("DRAWING...");
    {
        use embedded_graphics::draw_target::DrawTarget;
        use embedded_graphics::pixelcolor::Gray4;
        DrawTarget::clear(epd, Gray4::new(0xF));
    }
    epd_gfx::draw_header(&weather.time, "", epd)?;
    epd_gfx::draw_current_weather(&weather.weathercode, weather.temperature, epd)?;

    epd.update_screen(25i32);
    println!("DRAW COMPLETE");
    Ok(())
}
