use anyhow::Result;
use bytes::Bytes;
use core::time::Duration;
use embedded_svc::http::client::Response;
use embedded_svc::http::client::*;
use epd_gfx;
use epd_gfx::openmeteo::{self, Location, OpenMeteoConfig, OpenMeteoData};
use esp_idf_svc::http::client::EspHttpClient;
use std::thread;

pub mod epd;
pub mod epd_highlevel;
pub mod timer;
pub mod wifi;

#[no_mangle]
extern "C" fn app_main() {
    let main = init().unwrap();
    println!("init done");
    main.run();
}

fn init() -> Result<Main> {
    // Bind the log crate to the ESP Logging facilities
    // -> crashes :(
    //esp_idf_svc::log::EspLogger::initialize_default();

    let _t = timer::schedule_timer()?;
    let wifi = wifi::init()?;
    println!("wifi init done");
    let http_client = Box::new(EspHttpClient::new_default()?);
    println!("http client init done");
    let epd = epd::Epd::new();
    println!("epd init done");
    let meteo_config = OpenMeteoConfig::new(Location {
        lat: 48.93,
        lon: 8.4,
    });
    let weather_api_url = openmeteo::build_url(&meteo_config).clone();
    Ok(Main {
        weather_api_url,
        client: http_client,
        display: epd,
        wifi,
    })
}

struct Main {
    weather_api_url: String,
    client: Box<EspHttpClient>,
    display: epd::Epd,
    wifi: Box<EspWifi>,
}

impl Main {
    fn fetch_weather(&mut self) -> Result<OpenMeteoData> {
        let req = self.client.get(self.weather_api_url.clone())?;
        let resp = req.submit()?;
        // FIXME: check status code?
        let bytes: Result<Bytes, _> =
            embedded_svc::io::Bytes::<_, 64>::new(resp.reader()).collect();
        let body = bytes.map_err(anyhow::Error::msg)?;
        openmeteo::parse_reponse(&body)
    }

    fn run(mut self) -> ! {
        loop {
            self.fetch_google();
            match self.fetch_weather() {
                Ok(ref weather) => {
                    self.display.clear();
                    epd_gfx::draw(weather, &mut self.display)
                        .map_err(|e| println!("Error drawing screen: {e:?}"));
                    self.display.draw(25i32);
                }
                Err(e) => {
                    println!("Error fetching data: {e:?}");
                }
            }
            thread::sleep(Duration::from_secs(900));
        }
    }
}
