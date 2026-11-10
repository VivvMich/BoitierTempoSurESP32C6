use anyhow::Result;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Price {
    pub hp: f32,
    pub hc: f32,
}


#[derive(Debug, Deserialize)]
pub struct TempoDay {
    pub date: String,
    pub couleur: Option<String>,
    pub tarif: Option<Price>,
}

#[derive(Debug, Deserialize)]
pub struct RemainingDay {
    pub remaining: Option<Remaining>,
    pub totals: Option<Remaining>,
}

#[derive(Debug, Deserialize)]
pub struct Remaining {
    pub ROUGE: u32,
    pub BLANC: u32,
    pub BLEU: u32,
}

#[derive(Debug, Deserialize)]
pub struct RemainingDayTotals {
    pub ROUGE: u32,
    pub BLANC: u32,
    pub BLEU: u32,
}

#[derive(Debug, Deserialize)]
pub struct Prediction {
    pub predictions: Vec<ProbabilityNextDay>,
}
#[derive(Debug, Deserialize)]
pub struct ProbabilityNextDay {
    pub probabilite_bleu: f32,
    pub probabilite_blanc: f32,
    pub probabilite_rouge: f32,
}


pub fn today() -> Result<TempoDay> {
    let body = crate::http::get("https://www.calendrier-tempo.fr/api/today")?;
    Ok(serde_json::from_str(&body)?)
}

pub fn tomorrow() -> Result<TempoDay> {
    let body = crate::http::get("https://www.calendrier-tempo.fr/api/tomorrow")?;
    Ok(serde_json::from_str(&body)?)
}

pub fn remaining_red_days() -> Result<RemainingDay> {
    let body = crate::http::get("https://www.calendrier-tempo.fr/api/remaining")?;
    Ok(serde_json::from_str(&body)?)
}

pub fn predictions() -> Result<Prediction> {
    let body = crate::http::get("https://www.calendrier-tempo.fr/api/predictions")?;
    Ok(serde_json::from_str(&body)?)
}