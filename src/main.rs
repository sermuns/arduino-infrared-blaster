#![no_std]
#![no_main]

use arduino_hal::{delay_ms, delay_us, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

const TIME_UNIT_US: u32 = 562;

fn mark(tc2: &arduino_hal::pac::TC2, us: u32) {
    tc2.tccr2a().modify(|_, w| w.com2b().match_toggle());
    delay_us(us);
}

fn space(tc2: &arduino_hal::pac::TC2, us: u32) {
    tc2.tccr2a().modify(|_, w| w.com2b().disconnected());
    delay_us(us);
}

fn send_bit(tc2: &arduino_hal::pac::TC2, bit: bool) {
    mark(tc2, TIME_UNIT_US);

    if bit {
        space(tc2, TIME_UNIT_US * 3);
    } else {
        space(tc2, TIME_UNIT_US);
    }
}

fn send_u32(tc2: &arduino_hal::pac::TC2, mut v: u32) {
    for _ in 0..32 {
        let bit = (v & 1) != 0;
        send_bit(tc2, bit);
        v >>= 1;
    }
}

fn send_nec_frame(tc2: &arduino_hal::pac::TC2, addr: u32, cmd: u32) {
    mark(tc2, 9000);
    space(tc2, 4500);

    send_u32(tc2, addr);
    send_u32(tc2, cmd);

    mark(tc2, 562);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let tc2 = dp.TC2;
    let mut _d3 = pins.d3.into_output();

    tc2.tccr2a().write(|w| w.wgm2().ctc());
    tc2.tccr2b().write(|w| w.cs2().direct());
    // (try to) set 38 KHz, (do we need unsafe here???)
    tc2.ocr2a().write(|w| {
        // SAFETY:
        // it was revealed to me in a dream
        unsafe { w.bits(210) }
    });

    loop {
        uwriteln!(serial, "sending poweroff!").unwrap_infallible();
        for _ in 0..3 {
            send_nec_frame(&tc2, 0x86FF0000, 0x1BE40000);
            // send_nec_frame(&tc2, 0x02bd0000, 0x53ac0000);
            delay_ms(40);
        }
        delay_ms(200);
    }
}
