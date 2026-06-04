#![no_std]
#![no_main]

use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

mod sirc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut d3 = pins.d3.into_output();

    loop {
        uwriteln!(serial, "sending command").unwrap_infallible();

        for command in 0..=39 {
            sirc::send_sirc_command(&mut d3, 1, command);
        }

        delay_ms(45);
    }
}
