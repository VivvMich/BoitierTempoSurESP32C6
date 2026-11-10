use crate::{clock, tempo, temperature};
use crate::tempo::{Prediction, ProbabilityNextDay, TempoDay};

#[derive(Default)]
pub struct State {
    pub today: Option<TempoDay>,
    pub tomorrow: Option<TempoDay>,
    pub yesterday: Option<TempoDay>,
    pub temperature: Option<f32>,
    pub red_day: Option<u32>,
    pub prob: Option<Prediction>,
}



impl State {
    pub fn refresh(&mut self) {
        match tempo::today() {
            Ok(new) => {
                if let Some(old) = self.today.take() {
                    if old.date != new.date {
                        self.yesterday = Some(old);
                    } else {}
                }
                self.today = Some(new);
            }
            Err(e) => log::warn!("Tempo d'aujourd'hui : {e}"),
        }

        match tempo::tomorrow() {
            Ok(d) => self.tomorrow = Some(d),
            Err(e) => log::warn!("Tempo demain : {e}"),
        }

        match tempo::remaining_red_days() {
            Ok(rd) => { self.red_day = Some(rd.remaining.unwrap().ROUGE)}
            Err(e) => log::warn!("Reste jours rouges : {e}"),
        }

        match tempo::remaining_red_days() {
            Ok(rd) => { self.red_day = Some(rd.remaining.unwrap().ROUGE)}
            Err(e) => log::warn!("Reste jours rouges : {e}"),
        }

        match tempo::predictions() {
            Ok(p) => {
                log::info!("Predictions: {p:?}");
                self.prob = Some(p)
            }
            Err(e) => log::warn!("Prediction: {e}"),
        }

        match temperature::get_geo_data() {
        Ok(g) => {
            log::info!("Météo brute : {:?}", g.current);
            if let Some(t) = g.current.and_then(|c| c.temperature_2m) {
                self.temperature = Some(t);
            }
        }
            Err(e) => log::warn!("Météo : {e}")
        }
    }

    pub fn current_day(&self) -> Option<&TempoDay> {
        let c = clock::now().hour;
        if clock::before_change_day(c) {
            self.yesterday.as_ref().or(self.today.as_ref())
        } else {
            self.today.as_ref()
        }
    }


}