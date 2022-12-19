use std::fmt::Display;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub enum TrainType {
    ICE,
}

impl Display for TrainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TrainType::*;
        match self {
            ICE => write!(f, "ICE"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}
