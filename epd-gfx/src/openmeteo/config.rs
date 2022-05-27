use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenMeteoConfig {
    /// Geographical WGS84 coordinates of the location
    pub location: Location,

    pub hourly: Option<Vec<HourlyVariables>>,

    pub daily: Option<Vec<DailyVariables>>,
    current_weather: bool,
    /// celcius or fahrenheit
    temperature_unit: String,
    /// kmh, ms, mph or kn
    windspeed_unit: String,
    /// mm or inch
    precipitation_unit: String,
    /// iso8601 or unixtime
    /// If format unixtime is selected, all time values are returned in UNIX epoch time in seconds.
    /// Please not that all time is then in UTC! For daily values with unix timestamp, please apply
    /// utc_offset_seconds again to get the correct date.
    timeformat: String,
    /// If timezone is set, all timestamps are returned as local-time and data is returned starting
    /// at 0:00 local-time. Any time zone name from the time zone database is supported.
    timezone: String,

    /// (0-2) If past_days is set, yesterdays or the day before yesterdays data are also returned.
    past_days: u32,
}

impl OpenMeteoConfig {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            hourly: Some(vec![
                HourlyVariables::apparent_temperature,
                HourlyVariables::weathercode,
            ]),
            daily: None,
            current_weather: true,
            temperature_unit: "celcius".to_owned(),
            windspeed_unit: "kn".to_owned(),
            precipitation_unit: "mm".to_owned(),
            timeformat: "iso8601".to_owned(),
            timezone: "Europe/Berlin".to_owned(),
            past_days: 0,
        }
    }

    pub fn into_tuples(&self) -> Vec<(String, String)> {
        let mut params = vec![
            ("latitude".to_owned(), self.location.lat.to_string()),
            ("longitude".to_owned(), self.location.lon.to_string()),
            ("past_days".to_owned(), self.past_days.to_string()),
            (
                "timezone".to_owned(),
                self.timezone.to_string(), //.replace("/", "%2F").to_string(),
            ),
        ];
        if let Some(hourly_variables) = &self.hourly {
            params.extend(
                hourly_variables
                    .iter()
                    .map(|e| ("hourly".into(), e.to_string())),
            );
        }
        if let Some(daily_variables) = &self.daily {
            params.extend(
                daily_variables
                    .iter()
                    .map(|e| ("daily".into(), e.to_string())),
            );
        }
        if self.current_weather {
            params.push(("current_weather".into(), "true".into()));
        }
        return params;
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum HourlyVariables {
    temperature_2m,
    relativehumidity_2m,
    dewpoint_2m,
    apparent_temperature,
    pressure_msl,
    cloudcover,
    cloudcover_low,
    cloudcover_mid,
    cloudcover_high,
    windspeed_10m,
    windspeed_80m,
    windspeed_120m,
    windspeed_180m,
    winddirection_10m,
    winddirection_80m,
    winddirection_120m,
    winddirection_180m,
    windgusts_10m,
    shortwave_radiation,
    direct_radiation,
    direct_normal_irradiance,
    diffuse_radiation,
    vapor_pressure_deficit,
    evapotranspiration,
    precipitation,
    weathercode,
    snow_depth,
    freezinglevel_height,
    soil_temperature_0cm,
    soil_temperature_6cm,
    soil_temperature_18cm,
    soil_temperature_54cm,
    soil_moisture_0_1cm,
    soil_moisture_1_3cm,
    soil_moisture_3_9cm,
    soil_moisture_9_27cm,
    soil_moisture_27_81cm,
}

impl fmt::Display for HourlyVariables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum DailyVariables {
    temperature_2m_max,
    temperature_2m_min,
    apparent_temperature_max,
    apparent_temperature_min,
    precipitation_sum,
    precipitation_hours,
    weathercode,
    sunrise,
    windspeed_10m_max,
    winddirection_10m_dominant,
    shortwave_radiation_sum,
}
impl fmt::Display for DailyVariables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
