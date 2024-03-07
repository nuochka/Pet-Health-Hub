//! Main firmware binary of PHH aplication
//!
//! This binary is dedicated to ESP8266 (ESP-01) target, therefore special xtensa API is included
//! and used as a main processor library. PAC crate esp8266 is used for manipulations with mc
//! device.
//!
//! This binary sets the environment for reading from sensors and establishes connection with PHH
//! server and database.

#![allow(unreachable_code, unused_variables)]
#![no_std]
#![no_main]

use esp8266::Peripherals;               // ESP8266 PAC
use xtensa_lx_rt::{entry, pre_init};    // Startup code
use xtensa_lx as _;                     // Processor crate
use phh_esp_panic as _;                 // Panicking

/// Hardware initialization.
#[pre_init]
unsafe fn init() {
    let dp = Peripherals::take().unwrap();

    esp_println::logger::init_logger(log::LevelFilter::Debug);
    log::info!("Logger is setup");
}

/// Main kernel space.
#[entry]
fn main() -> ! {
    loop {}
}
