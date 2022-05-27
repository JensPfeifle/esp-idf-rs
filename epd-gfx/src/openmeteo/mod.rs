pub mod config;
pub mod responses;
use anyhow::{bail, Error};
use bytes::Bytes;
pub use config::{Location, OpenMeteoConfig};
pub use responses::{OpenMeteoData, OpenMeteoError};
use serde_repr::Deserialize_repr;
use std::fmt;

pub fn build_url(config: &OpenMeteoConfig) -> String {
    let base = "http://api.open-meteo.com/v1/forecast".to_owned();
    let query_params = config.into_tuples();
    let params = query_params
        .iter()
        .map(|(q, v)| format!("{q}={v}"))
        .map(|s| s.replace("/", "%2F"))
        .collect::<Vec<String>>();
    let url = base + "?" + &params.join("&");
    return url;
}

pub fn parse_reponse(bytes: &Bytes) -> Result<OpenMeteoData, Error> {
    if let Ok(data) = serde_json::from_slice::<OpenMeteoData>(bytes) {
        return Ok(data);
    }
    if let Ok(err) = serde_json::from_slice::<OpenMeteoError>(bytes) {
        bail!("OpenMeteo API returned error response: {:?}", err.reason);
    }
    bail!("Unable to decode response! {:?}", bytes);
}

#[derive(Clone, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum WMOCode {
    ClearSky = 0,
    MainlyClear = 1,
    PartyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    DepositingRimeFog = 48,
    LightDrizzle = 51,
    ModerateDrizzle = 53,
    DenseDrizzle = 55,
    LightFreezingDrizzle = 56,
    DenseFreezingDrizzle = 57,
    LightRain = 61,
    ModerateRain = 63,
    HeavyRain = 65,
    LightFreezingRain = 66,
    HeavyFreezingRain = 67,
    LightSnow = 71,
    ModerateSnow = 73,
    HeavySnow = 75,
    SnowGrains = 77,
    LightRainShowers = 80,
    ModerateRainShowers = 81,
    HeavyRainShowers = 82,
    LightSnowShowers = 85,
    HeavySnowShowers = 86,
    Thunderstorm = 95,
    ThunderstormWithLightHail = 96,
    ThunderstormWithHeavyHail = 99,
}

// derive format, to be able to call to_string()
impl fmt::Display for WMOCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
