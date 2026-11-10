use anyhow::Result;
use serde::Deserialize;
use crate::tempo::TempoDay;

#[derive(Debug, Deserialize)]
pub struct GeoData {
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: f32,
    pub current_units: Option<Units>,
    pub current: Option<CurrentTemperature>,
}
#[derive(Debug, Deserialize)]
pub struct Units {
    pub interval: Option<String>,
    pub temperature_2m: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CurrentTemperature {
    pub time: Option<String>,
    pub temperature_2m: Option<f32>,
}

pub fn get_geo_data() -> Result<GeoData> {
    let body = crate::http::get("https://api.open-meteo.com/v1/forecast?latitude=49.07&longitude=5.43&current=temperature_2m")?;
    Ok(serde_json::from_str(&body)?)
}