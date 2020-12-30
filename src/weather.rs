use serde::Deserialize;
use std::io;

// Melbourne
// const OBSERVATIONS_URL: &str = "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json";
// Sunshine Coast Airport http://www.bom.gov.au/products/IDQ60901/IDQ60901.94569.shtml
const OBSERVATIONS_URL: &str = "http://www.bom.gov.au/fwo/IDQ60901/IDQ60901.94569.json";

pub struct Client {}

#[derive(Debug, Deserialize)]
struct ObservationsRaw {
    observations: Observations,
}

#[derive(Debug, Deserialize)]
struct Observations {
    data: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
pub enum WindDirection {
    CALM,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    NNE,
    ENE,
    ESE,
    SSE,
    SSW,
    WSW,
    WNW,
    NNW,
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub sort_order: u32,
    pub name: String,
    pub history_product: String,
    pub local_date_time: String,      //"11/01:30pm",
    pub local_date_time_full: String, // "20180811133000",
    pub aifstime_utc: String,         // "20180811033000",
    pub lat: f64,
    pub lon: f64,
    pub apparent_t: f64,
    pub delta_t: f64,
    pub gust_kmh: u32,
    pub gust_kt: u32,
    pub air_temp: f64,
    pub dewpt: f64,
    pub press: Option<f64>,
    pub press_qnh: Option<f64>,
    pub press_msl: Option<f64>,
    pub press_tend: String,
    pub rain_trace: String, // Rain since 9am, not sure why this is a string
    pub rel_hum: u32,
    pub wind_dir: WindDirection,
    pub wind_spd_kmh: u32,
    pub wind_spd_kt: u32,
}

#[derive(Debug)]
pub enum WeatherError {
    HttpError,
    JsonError(io::Error),
}

//ftp://ftp.bom.gov.au/anon/gen/fwo/IDV10450.xml
//http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn observations(&self) -> Result<Vec<Observation>, WeatherError> {
        let resp = ureq::get(OBSERVATIONS_URL)
            .timeout(std::time::Duration::from_secs(10))
            .call();

        if resp.ok() {
            resp.into_json_deserialize::<ObservationsRaw>()
                .map(|obs| obs.observations.data)
                .map_err(WeatherError::JsonError)
        } else {
            Err(WeatherError::HttpError)
        }
    }
}
