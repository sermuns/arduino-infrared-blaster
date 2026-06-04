#![no_std]
#![no_main]

use arduino_hal::{delay_ms, delay_us, pac::TC2, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

// mod sirc;

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
        // 16 MHz / (1 * 2 * (OCR2A + 1 ) ) ~= 38 kHz
        w.set(209);
        w
    });

    loop {
        // uwriteln!(serial, "on").unwrap_infallible();
        // enable_carrier(&tc2);
        // delay_ms(500);
        // uwriteln!(serial, "off").unwrap_infallible();
        // disable_carrier(&tc2);

        uwriteln!(serial, "send message").unwrap_infallible();
        send_message(&tc2, 0x86FF, 0x1B);
        delay_ms(1000);
    }
}

fn enable_carrier(tc2: &TC2) {
    tc2.ocr2b().write(|w| w.set(209 / 3));
}

fn disable_carrier(tc2: &TC2) {
    tc2.ocr2b().write(|w| w.set(209));
}

fn send_one(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(560);
    disable_carrier(tc2);
    delay_us(2250 - 560);
}

fn send_zero(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(560);
    disable_carrier(tc2);
    delay_us(1120 - 560);
}

fn send_agc_burst(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(9000);
    disable_carrier(tc2);

    delay_us(4500);
}

fn send_final_burst(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(560);
    disable_carrier(tc2); // needed?
}

fn send_message(tc2: &TC2, address: u16, command: u8) {
    send_agc_burst(tc2);

    // (extended) address
    for i in (0..16).rev() {
        if (address >> i) & 1 != 0 {
            send_one(tc2);
        } else {
            send_zero(tc2);
        }
    }

    // command
    for i in (0..8).rev() {
        if (command >> i) & 1 != 0 {
            send_one(tc2);
        } else {
            send_zero(tc2);
        }
    }

    // inverted command
    for i in (0..8).rev() {
        if (command >> i) & 1 != 0 {
            send_zero(tc2);
        } else {
            send_one(tc2);
        }
    }

    send_final_burst(tc2);
}
