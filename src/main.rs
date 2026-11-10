
use anyhow::Result;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;

mod wifi;
mod http;
mod tempo;
mod temperature;
mod clock;
mod state;
mod display;

const SSID: &str = env!("WIFI_SSID");
const PASS: &str = env!("WIFI_PASS");

const UPDATE_INTERVAL: u64 = 60 * 30;


fn main() -> Result<()> {

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut screen = display::init(
        peripherals.spi2,
        peripherals.pins.gpio6,
        peripherals.pins.gpio7,
        peripherals.pins.gpio10,
        peripherals.pins.gpio11,
        peripherals.pins.gpio2,
        peripherals.pins.gpio3,
    )?;
    
    display::message(&mut screen, "Chargement...", Rgb565::WHITE);

    let _wifi = wifi::connect(peripherals.modem, sys_loop, nvs)?;
    let _sntp = clock::synchronise()?;
    let mut state = state::State::default();


    loop {
        state.refresh();
        display::drawing(&mut screen, &state);
        std::thread::sleep(std::time::Duration::from_secs(UPDATE_INTERVAL));
    }

}
