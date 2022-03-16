use anyhow::Result;
use brightsky::models::responses::CurrentWeatherResponse;
use reqwest;
use serde_json;

pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

pub fn fetch_current_weather(location: &Location) -> Result<CurrentWeatherResponse> {
    let url = "https://api.brightsky.dev/current_weather?lat=".to_owned()
        + &location.lat.to_string()
        + "&lon="
        + &location.lon.to_string();
    let body = reqwest::blocking::get(url)?.bytes()?;
    let data: CurrentWeatherResponse = serde_json::from_slice(&body)?;
    Ok(data)
}
