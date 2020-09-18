#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate cortex_m;
extern crate feather_m0 as hal;
extern crate panic_halt;
extern crate embedded_hal;

use embedded_hal::digital::v2::InputPin;
use hal::entry;
use hal::pac::Peripherals;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut parts = peripherals.PORT.split();
    let mut in_pin = parts.pa16.into_floating_input(&mut parts.port);

    if let Ok(_is_high) = in_pin.is_high() {
        // do something
    }

    loop { }
}