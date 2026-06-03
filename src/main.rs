#![no_std]
#![no_main]

use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // let timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(
    //     dp.TC2,
    //     arduino_hal::simple_pwm::Prescaler::Prescale8,
    // );

    let mut ir_led = pins.d3.into_output();

    loop {
        // send the power command over and over
    }
}
