//! Main firmware binary of PHH aplication
//!
//! This binary is dedicated to ESP8266 (ESP-01) target, therefore special xtensa API is included
//! and used as a main processor library. PAC crate esp8266 is used for manipulations with mc
//! device.

#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]

use panic_halt as _;        // Panicking
use xtensa_lx as _;         // Xtensa CPU features
use xtensa_lx_rt::entry;    // Xtensa CPU startup functions and vector mapping.

#[entry]
fn main() -> ! {
    loop {}
}
