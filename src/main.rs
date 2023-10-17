#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");

    // Set GPIO7 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let button = io.pins.gpio9.into_pull_up_input();
    let mut led = io.pins.gpio7.into_push_pull_output();
    led.set_high().unwrap();

    let mut i = 0;
    loop {
        if button.is_high().unwrap() {
            println!("Loop... {}", i);
            led.toggle().unwrap();
        }
        i += 1;
        delay.delay_ms(500u32);
    }
}