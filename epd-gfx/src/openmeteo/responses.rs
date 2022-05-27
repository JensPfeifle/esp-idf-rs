use crate::openmeteo::WMOCode;

use serde_derive::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OpenMeteoError {
    /// Always set true for errors.
    pub error: bool,
    /// Description of the error.
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OpenMeteoData {
    /// WGS84 of the center of the weather grid-cell which was used to generate this forecast. This
    /// coordinate might be up to 5 km away.
    pub latitude: f32,

    /// WGS84 of the center of the weather grid-cell which was used to generate this forecast. This
    /// coordinate might be up to 5 km away.
    pub longitude: f32,

    /// The elevation in meters of the selected weather grid-cell. In mountain terrain it might
    /// differ from the location you would expect.
    pub elevation: f32,

    /// Generation time of the weather forecast in milli seconds. This is mainly used for
    /// performance monitoring and improvements.
    pub generationtime_ms: f32,

    /// Applied timezone offset from the &timezone= parameter.
    pub utc_offset_seconds: i32,

    /// For each selected weather variable, data will be returned as a floating point array.
    /// Additionally a `time` array will be returned with ISO8601 timestamps.
    pub hourly: Option<OpenMeteoHourlyVariables>,
    /// For each selected weather variable, the unit will be listed here.
    pub hourly_units: Option<serde_json::Value>,

    /// For each selected daily weather variable, data will be returned as a floating point array.
    /// Additionally a `time` array will be returned with ISO8601 timestamps.
    pub daily: Option<OpenMeteoDailyVariables>,
    /// For each selected daily weather variable, the unit will be listed here.
    pub daily_units: Option<serde_json::Value>,

    /// Current weather conditions.
    pub current_weather: Option<OpenMeteoCurrentWeather>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OpenMeteoHourlyVariables {
    /// Timestamp for each entry.
    pub time: Vec<String>,
    /// Air temperatures at 2 meters above ground.
    pub temperature_2m: Option<Vec<f32>>,

    ///Relative humidity at 2 meters above ground [%]
    pub relativehumidity_2m: Option<Vec<f32>>,

    ///Dew point temperature at 2 meters above ground [°C (°F)]
    pub dewpoint_2m: Option<Vec<f32>>,

    ///C (°F)  Apparent temperature is the perceived feels-like tempertature combinding wind chill factor, realtive humidity and solar radition []
    pub apparent_temperature: Option<Vec<f32>>,

    ///Atmospheric air pressure reduced to sea level [hPa]
    pub pressure_msl: Option<Vec<f32>>,

    ///Total cloud cover as an area fraction [%]
    pub cloudcover: Option<Vec<f32>>,

    ///Low level clouds and fog up to 3 km altitude [%]
    pub cloudcover_low: Option<Vec<f32>>,

    ///Mid level clouds from 3 to 8 km altitude [%]
    pub cloudcover_mid: Option<Vec<f32>>,

    ///High level clouds from 8 km altitude [%]
    pub cloudcover_high: Option<Vec<f32>>,
    pub windspeed_10m: Option<Vec<f32>>,
    pub windspeed_80m: Option<Vec<f32>>,
    pub windspeed_120m: Option<Vec<f32>>,

    /// Wind speed at 10, 80, 120 or 180 meters above ground. Wind speed on 10 meters is the
    /// standard level. [(max  km/h (mph, m/s, knots)]
    pub windspeed_180m: Option<Vec<f32>>,
    pub winddirection_10m: Option<Vec<f32>>,
    pub winddirection_80m: Option<Vec<f32>>,
    pub winddirection_120m: Option<Vec<f32>>,

    ///Wind direction at 10, 80, 120 or 180 meters above ground [°]
    pub winddirection_180m: Option<Vec<f32>>,

    /// Gusts at 10 meters above ground as a maximum of the preceding hour [(max  km/h (mph, m/s, knots)]
    pub windgusts_10m: Option<Vec<f32>>,

    ///Shortwave solar radiation as average of the preceding hour [W/m²]
    pub shortwave_radiation: Option<Vec<f32>>,
    /// [W/m²]
    pub direct_radiation: Option<Vec<f32>>,

    /// Direct solar radiation as average of the preceding hour on the horizontal plane and the
    /// normal plane (perpendicular to the sun) [W/m²]
    pub direct_normal_irradiance: Option<Vec<f32>>,

    /// Diffuse solar radiation as average of the preceding hour [hour]
    pub diffuse_radiation: Option<Vec<f32>>,

    /// Vapor Pressure Deificit (VPD) in kilo pascal (kPa). For high VPD (>1.6), water
    /// transpiration of plants increases. For low VPD (<0.4), transpiration decreases [kPa]
    pub vapor_pressure_deficit: Option<Vec<f32>>,

    ///Sum of evapotranspration of the preceding hour from land surface and plants.
    pub evapotranspiration: Option<Vec<f32>>,

    /// Total precipitation (rain, showers, snow) sum of the preceding hour.
    pub precipitation: Option<Vec<f32>>,

    /// Weather condition as a numeric code. Follow WMO weather interpretation codes.
    pub weathercode: Option<Vec<WMOCode>>,

    /// Snow depth on the ground [meters]
    pub snow_depth: Option<Vec<f32>>,

    /// Altitude above sea level of the 0°C level.
    pub freezinglevel_height: Option<Vec<f32>>,
    pub soil_temperature_0cm: Option<Vec<f32>>,
    pub soil_temperature_6cm: Option<Vec<f32>>,
    pub soil_temperature_18cm: Option<Vec<f32>>,

    /// Temperature in the soil at 0, 6, 18 and 54 cm depths. 0 cm is the surface temperature on
    /// land or water surface temperature on water.
    pub soil_temperature_54cm: Option<Vec<f32>>,
    pub soil_moisture_0_1cm: Option<Vec<f32>>,
    pub soil_moisture_1_3cm: Option<Vec<f32>>,
    pub soil_moisture_3_9cm: Option<Vec<f32>>,
    pub soil_moisture_9_27cm: Option<Vec<f32>>,

    /// Average soil water content as volumetric mixing ratio at 0-1, 1-3, 3-9, 9-27 and 27-81 cm
    /// depths.
    pub soil_moisture_27_81cm: Option<Vec<f32>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OpenMeteoDailyVariables {
    /// Timestamp for each entry.
    pub time: Vec<String>,
    ///Maximum and minimum daily air temperature at 2 meters above ground
    pub temperature_2m_max: Option<Vec<f32>>,
    ///Maximum and minimum daily air temperature at 2 meters above ground
    pub temperature_2m_min: Option<Vec<f32>>,
    /// 	Maximum and minimum dailt apparent temperature
    pub apparent_temperature_max: Option<Vec<f32>>,
    /// 	Maximum and minimum dailt apparent temperature
    pub apparent_temperature_min: Option<Vec<f32>>,
    ///Sum of daily precipitation
    pub precipitation_sum: Option<Vec<f32>>,
    ///	The number of hours with rain
    pub precipitation_hours: Option<Vec<f32>>,
    ///The most severe weather condition on a given day as WMO code.
    pub weathercode: Option<WMOCode>,
    /// Sunrise time (iso8601).
    pub sunrise: Option<Vec<String>>,
    /// Sunset time (iso8601).
    pub sunset: Option<Vec<String>>,
    ///Maximum wind speed on a day km/h (mph, m/s, knots)
    pub windspeed_10m_max: Option<Vec<f32>>,
    ///Maximum wind gusts on a day km/h (mph, m/s, knots)
    pub windgusts_10m_max: Option<Vec<f32>>,
    /// Dominant wind direction [°]
    pub winddirection_10m_dominant: Option<Vec<u32>>,
    ///The sum of solar radiaion on a given day in Mega Joules [MJ/m²]
    pub shortwave_radiation_sum: Option<Vec<f32>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OpenMeteoCurrentWeather {
    pub time: String,
    pub temperature: f32,
    pub windspeed: f32,
    pub winddirection: u32,
    pub weathercode: WMOCode,
}
