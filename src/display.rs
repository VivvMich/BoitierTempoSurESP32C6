use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use anyhow::Result;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::{Rectangle, PrimitiveStyle}, };
use embedded_graphics::mono_font::{iso_8859_15::FONT_10X20, MonoTextStyle};
use embedded_graphics::primitives::Line;
use esp_idf_svc::hal::{
    delay::Delay,
    gpio::{AnyInputPin, Gpio10, Gpio11, Gpio2, Gpio3, Gpio6, Gpio7, PinDriver},
    spi::{
        config::{Config, DriverConfig},
        SpiDeviceDriver, SPI2,
    },
    units::FromValueType,
};
use mipidsi::{interface::SpiInterface, models::ST7789, options::ColorInversion, Builder};
use mipidsi::options::{Orientation, Rotation};
use crate::tempo::TempoDay;
use crate::{clock, UPDATE_INTERVAL};
use crate::state::State;

pub fn init(
    spi: SPI2<'static>,
    sclk: Gpio6<'static>,
    mosi: Gpio7<'static>,
    cs: Gpio10<'static>,
    dc: Gpio11<'static>,
    rst: Gpio2<'static>,
    bl: Gpio3<'static>,
) -> Result<impl DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>> {
    let mut backlight = PinDriver::output(bl)?;
    backlight.set_high()?;
    core::mem::forget(backlight); // sinon la broche est relâchée à la fin de la fonction

let spi_dev = SpiDeviceDriver::new_single(
    spi,
    sclk,
    mosi,
    None::<AnyInputPin>,
    Some(cs),
    &DriverConfig::new(),
    &Config::new().baudrate(40.MHz().into()),
)?;

    let dc = PinDriver::output(dc)?;
    let rst = PinDriver::output(rst)?;

    let buffer: &'static mut [u8] = Box::leak(Box::new([0u8; 512]));
    let di = SpiInterface::new(spi_dev, dc, buffer);

    let mut delay = Delay::new_default();
    let display = Builder::new(ST7789, di)
        .display_size(240, 320)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .map_err(|e| anyhow::anyhow!("Init écran : {:?}", e))?;
    Ok(display)
}

fn colors(nom: Option<&str>) -> (Rgb565, Rgb565) {
    match nom {
        Some("BLEU") => (Rgb565::BLUE, Rgb565::WHITE),
        Some("BLANC") => (Rgb565::WHITE, Rgb565::BLACK),
        Some("ROUGE") => (Rgb565::RED, Rgb565::WHITE),
        _ => (Rgb565::new(15, 31, 15), Rgb565::WHITE), // gris : inconnu
    }
}

 fn date_fr(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() == 3 {
        format!("{}/{}/{}", parts[2], parts[1], parts[0])
    } else {
        iso.to_string() // format inattendu : on affiche tel quel
    }
}


fn block<D: DrawTarget<Color = Rgb565>>(display: &mut D, x:i32, title: &str, day: Option<&TempoDay>) {
    let name = day.and_then(|d| d.couleur.as_deref());
    let (background, text) = colors(name);
    let style = MonoTextStyle::new(&FONT_10X20, text);

    let _ = Rectangle::new(Point::new(x, 0), Size::new(160, 200))
    .into_styled(PrimitiveStyle::with_fill(background))
    .draw(display);


    // Titre ( aujourd'hui demain etc ...)
    let _ = Text::new(title, Point::new(x + 10, 30), style).draw(display);
    // Date

    let date = date_fr(&day.unwrap().date);

    let _ = Text::new(date.as_str(), Point::new(x + 10, 50), style).draw(display);
    let _ = Text::new(name.unwrap_or("INDISPONIBLE"), Point::new(x + 10,  80), style).draw(display);

    if let Some(p) = day.and_then(|d| d.tarif.as_ref()) {
        let _ = Text::new(&format!("HP {:.4} €/H", p.hp), Point::new(x + 10, 110), style).draw(display);
        let _ = Text::new(&format!("HC {:.4} €/H", p.hc), Point::new(x + 10, 140), style).draw(display);
    }



}

pub fn drawing<D: DrawTarget<Color = Rgb565>>(d: &mut D, state: &State) {
    block(d, 0, "AUJOURD'HUI", state.current_day());
    block(d, 160, "DEMAIN", state.tomorrow.as_ref());


    // Bandeau
    let _ = Rectangle::new(Point::new(0, 160), Size::new(320, 80))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(d);

    let _ = Line::new(Point::new(160, 0), Point::new(160, 160))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(d);

    let remaining_days = match state.red_day {
        Some(rd) => format!("{}", rd),
        None => "?".to_string(),
    };

    let r_style = MonoTextStyle::new(&FONT_10X20, Rgb565::RED);
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let b_style = MonoTextStyle::new(&FONT_10X20, Rgb565::BLUE);

    let _ = Text::new(
        &format!("{} / 22 jour(s) restant(s)", remaining_days),
        Point::new(10, 180),
        r_style,
    )
        .draw(d);

    let mut blue: u32 = 0;
    let mut white: u32 = 0;
    let mut red: u32 = 0;


    if let Some(p) =  &state.prob {
        blue =  ( p.predictions[1].probabilite_bleu * 100.0 ) as u32;
        white =  ( p.predictions[1].probabilite_blanc * 100.0 ) as u32;
        red =  ( p.predictions[1].probabilite_rouge * 100.0) as u32;
    }

    let _ = Text::new(
        &format!("{} %", blue),
        Point::new(10, 205),
        b_style,
    )
        .draw(d);

    let _ = Text::new(
        &format!("{} %", white),
        Point::new(70, 205),
        style,
    )
        .draw(d);

    let _ = Text::new(
        &format!("{} %", red),
        Point::new(130, 205),
        r_style,
    )
        .draw(d);


    // Bandeau du bas : température, tarif en cours, heure

    let n = clock::now();
    //let tarif = if clock::hollows_hours(n.hour) { "HC" } else { "HP" };
    let temp = match state.temperature {
        Some(t) => format!("{:.1} °C", t),
        None => "-- °C".to_string(),
    };

    let _ = Text::new(
        &format!("{} | Mise à jour de {:02}:{:02}", temp, n.hour, n.min),
        Point::new(10, 235),
        style,
    )
        .draw(d);
}

/// Écran noir avec un message centré
pub fn message<D: DrawTarget<Color = Rgb565>>(d: &mut D, texte: &str, couleur: Rgb565) {
    let _ = d.clear(Rgb565::BLACK);
    let style = MonoTextStyle::new(&FONT_10X20, couleur);
    let alignement = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();
    let _ = Text::with_text_style(texte, Point::new(160, 120), style, alignement).draw(d);
}