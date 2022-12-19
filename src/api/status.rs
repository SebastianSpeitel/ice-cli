use super::common::*;
use serde_derive::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Status {
    pub connection: bool,
    #[serde(rename = "serviceLevel")]
    pub service_level: ServiceLevel,
    #[serde(rename = "gpsStatus")]
    pub gps_status: GpsStatus,
    pub internet: Internet,
    #[serde(flatten)]
    pub geocoordinates: GeoCoordinates,
    #[serde(rename = "tileY")]
    pub tile_y: i64,
    #[serde(rename = "tileX")]
    pub tile_x: i64,
    pub series: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    pub speed: f64,
    #[serde(rename = "trainType")]
    pub train_type: TrainType,
    pub tzn: String,
    #[serde(rename = "wagonClass")]
    pub wagon_class: Class,
    pub connectivity: Connectivity,
    #[serde(rename = "bapInstalled")]
    pub bap_installed: bool,
}

#[derive(Deserialize, Debug)]
pub enum ServiceLevel {
    #[serde(rename = "AVAILABLE_SERVICE")]
    Available,
}

#[derive(Deserialize, Debug)]
pub enum GpsStatus {
    #[serde(rename = "VALID")]
    Valid,
}

#[derive(Deserialize, Debug)]
pub enum Internet {
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Deserialize, Debug)]
pub enum Class {
    #[serde(rename = "FIRST")]
    First,
    #[serde(rename = "SECOND")]
    Second,
}

#[derive(Deserialize, Debug)]
pub struct Connectivity {
    #[serde(rename = "currentState")]
    pub current_state: ConnectivityState,
    #[serde(rename = "nextState")]
    pub next_state: ConnectivityState,
    #[serde(rename = "remainingTime")]
    pub remaining_time: Option<Duration>,
}

#[derive(Deserialize, Debug)]
pub enum ConnectivityState {
    #[serde(rename = "UNSTABLE")]
    Unstable,
    #[serde(rename = "HIGH")]
    High,
}
