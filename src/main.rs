#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms, delay_ns, delay_us,
    port::{Pin, PinOps, mode::Output},
    prelude::*,
};
use panic_halt as _;
use ufmt::uwriteln;

const TIME_BETWEEN_EDGE_NS: u32 = 10_u32.pow(9) / 38_000;

const UNIT_BURST_LENGTH_US: u32 = 600;
const ZERO_BURST_LENGHT_US: u32 = UNIT_BURST_LENGTH_US;
const ONE_BURST_LENGTH_US: u32 = 2 * UNIT_BURST_LENGTH_US;
const START_BURST_LENGTH_US: u32 = 4 * 600;

fn send_one(pin: &mut Pin<Output, impl PinOps>) {
    const NUM_ITERATIONS: u32 = ONE_BURST_LENGTH_US * 1000 / TIME_BETWEEN_EDGE_NS;

    for _ in 0..NUM_ITERATIONS {
        pin.toggle();
        delay_ns(TIME_BETWEEN_EDGE_NS);
    }

    pin.set_low();
}

fn send_zero(pin: &mut Pin<Output, impl PinOps>) {
    pin.set_low();
    delay_us(ZERO_BURST_LENGHT_US);
}
use send_zero as send_space;

fn send_start(pin: &mut Pin<Output, impl PinOps>) {
    const NUM_ITERATIONS: u32 = START_BURST_LENGTH_US * 1000 / TIME_BETWEEN_EDGE_NS;

    for _ in 0..NUM_ITERATIONS {
        pin.toggle();
        delay_ns(TIME_BETWEEN_EDGE_NS);
    }

    pin.set_low();
}

fn send_sirc_command(pin: &mut Pin<Output, impl PinOps>, address: u8, command: u8) {
    send_start(pin);

    send_space(pin);

    for i in 0..7 {
        let bit = (command >> i) & 1;
        if bit == 1 {
            send_one(pin);
        } else {
            send_zero(pin);
        }
        send_space(pin);
    }

    for i in 0..5 {
        let bit = (address >> i) & 1;
        if bit == 1 {
            send_one(pin);
        } else {
            send_zero(pin);
        }
        send_space(pin);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut d3 = pins.d3.into_output();

    loop {
        uwriteln!(serial, "sending command").unwrap_infallible();

        send_sirc_command(&mut d3, 1, 19);

        delay_ms(45);
    }
}
