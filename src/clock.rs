use anyhow::{Result, bail};
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use std::time::Duration;

pub fn synchronise() -> Result<EspSntp<'static>> {
    std::env::set_var("TZ", "CET-1CEST,M3.5.0,M10.5.0/3");
    unsafe { esp_idf_svc::sys::tzset() };

    let sntp = EspSntp::new_default()?;
    for _ in 0..30 {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        log::info!("Epoch : {}", secs);
        if secs > 1_700_000_000 {
            log::info!("Heure synchronisée");
            return Ok(sntp);
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    bail!("SNTP : heure toujours invalide après 30 s")
}

pub struct Now {
    pub hour: u32,
    pub min: u32,
}

pub fn now() -> Now {
    unsafe {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0) as esp_idf_svc::sys::time_t;

        let mut tm: esp_idf_svc::sys::tm = std::mem::zeroed();
        esp_idf_svc::sys::localtime_r(&secs, &mut tm);
        log::info!("secs={} tm_hour={}", secs, unsafe { tm.tm_hour });
        Now {
            hour: tm.tm_hour as u32,
            min: tm.tm_min as u32,
        }

    }
}

pub fn hollows_hours(hour: u32) -> bool {
    hour >= 22 || hour < 6
}

pub fn before_change_day(hour: u32) -> bool {
    hour < 6
}