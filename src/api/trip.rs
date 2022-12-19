use super::common;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TripInfo {
    pub trip: Trip,
    pub connection: Option<Connection>,
    pub active: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Trip {
    #[serde(rename = "tripDate")]
    pub trip_date: String,
    #[serde(rename = "trainType")]
    pub train_type: common::TrainType,
    pub vzn: String,
    #[serde(rename = "actualPosition")]
    pub actual_position: i32,
    #[serde(rename = "distanceFromLastStop")]
    pub distance_from_last_stop: i32,
    #[serde(rename = "totalDistance")]
    pub total_distance: i32,
    #[serde(rename = "stopInfo")]
    pub stop_info: StopInfo,
    pub stops: Vec<stop::Stop>,
}

#[derive(Deserialize, Debug)]
pub struct StopInfo {
    #[serde(rename = "scheduledNext")]
    pub scheduled_next: String,
    #[serde(rename = "actualNext")]
    pub actual_next: String,
    #[serde(rename = "actualLast")]
    pub actual_last: String,
    #[serde(rename = "actualLastStarted")]
    pub actual_last_started: String,
    #[serde(rename = "finalStationName")]
    pub final_station_name: String,
    #[serde(rename = "finalStationEvaNr")]
    pub final_station_eva_nr: String,
}

mod stop {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Stop {
        pub station: Station,
        pub timetable: Timetable,
        pub track: Track,
        pub info: Info,
        #[serde(rename = "delayReasons")]
        pub delay_reasons: Option<Vec<DelayReason>>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Station {
        #[serde(rename = "evaNr")]
        pub eva_nr: String,
        pub name: String,
        pub code: Option<String>,
        pub geocoordinates: common::GeoCoordinates,
    }

    #[derive(Deserialize, Debug)]
    pub struct Timetable {
        #[serde(rename = "scheduledArrivalTime")]
        pub scheduled_arrival_time: Option<i64>,
        #[serde(rename = "actualArrivalTime")]
        pub actual_arrival_time: Option<i64>,
        #[serde(rename = "showActualArrivalTime")]
        pub show_actual_arrival_time: Option<bool>,
        #[serde(rename = "arrivalDelay")]
        pub arrival_delay: String,
        #[serde(rename = "scheduledDepartureTime")]
        pub scheduled_departure_time: Option<i64>,
        #[serde(rename = "actualDepartureTime")]
        pub actual_departure_time: Option<i64>,
        #[serde(rename = "showActualDepartureTime")]
        pub show_actual_departure_time: Option<bool>,
        #[serde(rename = "departureDelay")]
        pub departure_delay: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Track {
        pub scheduled: String,
        pub actual: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Info {
        pub status: i32,
        pub passed: bool,
        #[serde(rename = "positionStatus")]
        pub position_status: PositionStatus,
        pub distance: i32,
        #[serde(rename = "distanceFromStart")]
        pub distance_from_start: i32,
    }

    #[derive(Deserialize, Debug)]
    pub enum PositionStatus {
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "departed")]
        Departed,
        #[serde(rename = "future")]
        Future,
    }

    #[derive(Deserialize, Debug)]
    pub struct DelayReason {
        pub code: String,
        pub text: String,
    }
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    #[serde(rename = "trainType")]
    pub train_type: Option<common::TrainType>,
    pub vzn: Option<String>,
    #[serde(rename = "trainNumber")]
    pub train_number: Option<String>,
    pub station: Option<String>,
    pub timetable: Option<String>,
    pub track: Option<String>,
    pub info: Option<String>,
    pub stops: Option<String>,
    pub conflict: String,
}
