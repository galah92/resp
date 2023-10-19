#![no_std]
#![no_main]

use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_backtrace as _;
use esp_println::println;
use esp_wifi::wifi::WifiMode;
use esp_wifi::{initialize, EspWifiInitFor};
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};
use hal::{systimer::SystemTimer, Rng};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    println!("Hello world!");

    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let (wifi, _) = peripherals.RADIO.split();
    let (_wifi_interface, mut controller) =
        esp_wifi::wifi::new_with_mode(&init, wifi, WifiMode::Sta).unwrap();
    println!("Device capabilities: {:?}", controller.get_capabilities());

    const SSID: &str = env!("SSID");
    const PASSWORD: &str = env!("PASSWORD");
    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        password: PASSWORD.into(),
        ..Default::default()
    });
    println!("Setting configuration: {:?}", &client_config);
    controller.set_configuration(&client_config).unwrap();
    controller.start().unwrap();
    println!("Wifi started: {}", controller.is_started().unwrap());
    controller.connect().unwrap();

    // Set GPIO7 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let button = io.pins.gpio9.into_pull_up_input();
    let mut led = io.pins.gpio7.into_push_pull_output();
    led.set_high().unwrap();

    let mut i = 0;
    loop {
        if button.is_high().unwrap() {
            println!("Loop... {} {:?}", i, controller.is_connected());
            led.toggle().unwrap();
        }
        i += 1;
        delay.delay_ms(500u32);
    }
}
