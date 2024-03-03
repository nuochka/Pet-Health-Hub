#![feature(custom_test_frameworks, used_with_arg)]
#![allow(non_snake_case)]
#![no_std]

//! Main library entry for PHH firmware for esp8266ex (esp01) target.
//!
//! This library provides all functionality related to PHH service. All sensors which are
//! supported by the target. This library do not guarantee the functionality on all espressif
//! devices and so must be compiled and written to the board by an installer.

use xtensa_lx::interrupt::InterruptNumber;

#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

/// Enumeration of all interrupts.
#[derive(Debug, Clone, Copy)]
pub enum Interrupt {}


unsafe impl InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        match self {}
    }
}
