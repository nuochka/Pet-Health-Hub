//! Main firmware binary of PHH aplication
//!
//! This binary is dedicated to ESP32 target. The HAL is mainly used for initiazation purposes.
//!
//! This binary sets the environment for reading from sensors and establishes connection with PHH
//! server and database.

#![no_std]
#![no_main]
#![allow(unreachable_code, unused_variables)]

use esp32_hal::{
    prelude::*,
    peripherals::Peripherals,
};
use esp_backtrace as _;

/// Hardware initialization.
#[entry]
fn init() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    esp_println::logger::init_logger(log::LevelFilter::Debug);
    log::info!("Logger is setup");

    main(); // Main kernel.

    loop {}
}

/// Main kernel space.
fn main() -> ! {
    loop {}
}
