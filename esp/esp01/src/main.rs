//! Main firmware binary of PHH aplication
//!
//! This binary is dedicated to ESP8266 (ESP-01) target, therefore special xtensa API is included
//! and used as a main processor library. PAC crate esp8266 is used for manipulations with mc
//! device.

#![feature(custom_test_frameworks)]
#![allow(dead_code)]
#![no_std]
#![no_main]

use core::cell::RefCell;

use esp8266 as device;                  // Target device PAC, created with svd2rust.

use device::GPIO;                       // Device GPIO.

use xtensa_lx_rt::{entry, pre_init};    // Xtensa CPU startup functions and vector mapping.
use xtensa_lx::{                        // Xtensa CPU features
    self,
    interrupt,
};
use phh_esp_panic as _;                 // Panicking

const GPIO: RefCell<Option<GPIO>> = RefCell::new(None);

/// Initialization code for the application.
#[pre_init]
unsafe fn init() {
    let dp = device::Peripherals::take().unwrap(); // Startup initialization.

    let gpio = dp.GPIO;        // Main GPIO pins.
    

    interrupt::free(|_cs| {
        gpio.gpio_enable.write(|w| 
            w.gpio_enable_data().bits(0xffff)
        );

        gpio.gpio_out.write(|w|
            w.gpio_out_data().bits(0xffff)
        );

        GPIO.borrow_mut().replace(gpio);    
    });
}

/// Main firmware kernel code.
#[entry]
fn main() -> ! {
    let mut gpio = GPIO;

    gpio.get_mut().as_mut().unwrap().gpio_in.write(|w|
        unsafe { w.bits(0xffff) }
    ); 

    loop {}
}
