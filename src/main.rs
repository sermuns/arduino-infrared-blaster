#![no_std]
#![no_main]

use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

fn enable_carrier(tc2: &arduino_hal::pac::TC2) {
    tc2.tccr2a().modify(|_, w| {
        w.com2b().match_toggle();
        w
    });
}

fn disable_carrier(tc2: &arduino_hal::pac::TC2) {
    tc2.tccr2a().modify(|_, w| {
        w.com2b().disconnected();
        w
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let tc2 = dp.TC2;
    let _d3 = pins.d3.into_output();

    tc2.tccr2a().write(|w| {
        w.wgm2().ctc();
        w
    });

    tc2.tccr2b().write(|w| {
        w.cs2().direct();
        w
    });

    // (try to) set 38 KHz
    tc2.ocr2a().write(|w| unsafe { w.bits(210) });

    loop {
        enable_carrier(&tc2);
        uwriteln!(serial, "Carrier enabled").unwrap_infallible();
        delay_ms(1000);

        disable_carrier(&tc2);
        uwriteln!(serial, "Carrier disabled").unwrap_infallible();
        delay_ms(1000);
    }
}
