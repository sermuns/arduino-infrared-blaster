#![no_std]
#![no_main]

use arduino_hal::{delay_ms, pac::TC2, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

mod sirc;
// mod necext;

const CLOCK_FREQ_KHZ: u32 = 16_000;
const CARRIER_FREQUENCY_KHZ: u32 = 40;
const OCR2A_VALUE: u8 = (CLOCK_FREQ_KHZ / (2 * CARRIER_FREQUENCY_KHZ) - 1) as u8;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut _d3 = pins.d3.into_output();

    let tc2 = dp.TC2;

    tc2.tccr2a().write(|w| {
        w.wgm2().pwm_fast();
        w.com2b().match_set();
        w
    });
    tc2.tccr2b().write(|w| {
        w.wgm22().set_bit();
        w.cs2().direct();
        w
    });
    tc2.ocr2a().write(|w| {
        w.set(OCR2A_VALUE);
        w
    });

    loop {
        // uwriteln!(serial, "on").unwrap_infallible();
        // enable_carrier(&tc2);
        // delay_ms(500);
        //
        // uwriteln!(serial, "off").unwrap_infallible();
        // disable_carrier(&tc2);
        // delay_ms(500);

        uwriteln!(serial, "ocr2a: {}", OCR2A_VALUE).unwrap_infallible();
        uwriteln!(serial, "send message").unwrap_infallible();
        // necext::send_message(&tc2, 0x86FF, 0x1B);
        sirc::send_message(&tc2, 0x01, 0x15);
        delay_ms(45); // FIXME: wrong
    }
}

pub fn enable_carrier(tc2: &TC2) {
    tc2.ocr2b().write(|w| w.set(OCR2A_VALUE / 3));
}

pub fn disable_carrier(tc2: &TC2) {
    tc2.ocr2b().write(|w| w.set(OCR2A_VALUE));
}
