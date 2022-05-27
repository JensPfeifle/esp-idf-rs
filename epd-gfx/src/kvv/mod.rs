use serde::{self, Deserialize};
use serde_aux::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Response {
    #[serde(rename = "departureList")]
    pub departures: Vec<Departure>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DateTime {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    year: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    month: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    day: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    weekday: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    hour: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    minute: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    // minutes until departure
    countdown: String,
    // scheduled date and time of stop
    #[serde(rename = "dateTime")]
    datetime: DateTime,
    // actual predicted date and time of stop
    // only if realtime_trip_status is "MONITORED"
    #[serde(rename = "realDateTime")]
    real_datetime: Option<DateTime>,
    // if relatime infomration is available
    #[serde(rename = "realtimeTripStatus")]
    realtime_trip_status: Option<String>,
    // information about the train or bus line
    #[serde(rename = "servingLine")]
    line: LineInformation,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct LineInformation {
    // Number/route ID
    number: String,
    // end of the line, usually shown on the train/bus
    direction: String,
    // start of the line
    #[serde(rename = "directionFrom")]
    direction_from: String,
    // "S-Bahn" or "Regionalbus"
    name: String,
    // "1" if realtime information is available
    realtime: String,
    // Delay in minutes
    delay: Option<String>,
}
