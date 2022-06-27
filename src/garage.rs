use serde::Deserialize;

use crate::weather::WeatherError;

const GARAGE_URL: &str = "http://10.0.0.11:8888/door.json";

pub struct Client {}

//{
//  "state": "Closed",
//  "secs_since_notified": null,
//  "open_for": null
//}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Door {
    pub state: DoorState,
    secs_since_notified: Option<u64>,
    open_for: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
    Unknown,
}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn state(&self) -> Result<Door, WeatherError> {
        let resp = ureq::get(GARAGE_URL)
            .timeout(std::time::Duration::from_secs(10))
            .call();

        resp.map_err(|_err| WeatherError::HttpError)
            .and_then(|resp| resp.into_json::<Door>().map_err(WeatherError::JsonError))
    }
}
