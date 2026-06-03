#![no_std]
#![no_main]

use arduino_hal::{delay_ms, delay_us, prelude::*};
use panic_halt as _;
use ufmt::uwriteln;

const TIME_UNIT_US: u32 = 562;

fn mark(tc2: &arduino_hal::pac::TC2, us: u32) {
    tc2.tccr2a().modify(|_, w| {
        w.com2b().match_toggle();
        w
    });
    delay_us(us);
}

fn space(tc2: &arduino_hal::pac::TC2, us: u32) {
    tc2.tccr2a().modify(|_, w| {
        w.com2b().disconnected();
        w
    });
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

fn send_nec_frame(tc2: &arduino_hal::pac::TC2, addr: u32, cmd: u32) {
    mark(tc2, TIME_UNIT_US * 16);
    space(tc2, TIME_UNIT_US * 8);

    for i in 0..32 {
        let bit = (addr >> i) & 1;
        send_bit(tc2, bit != 0);
    }

    for i in 0..32 {
        let bit = (cmd >> i) & 1;
        send_bit(tc2, bit != 0);
    }

    mark(tc2, TIME_UNIT_US);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let tc2 = dp.TC2;
    let mut d3 = pins.d3.into_output();

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
        uwriteln!(serial, "sending poweroff!").unwrap_infallible();
        send_nec_frame(&tc2, 0x86FF0000, 0x1BE40000);
        delay_ms(1000);
    }
}
