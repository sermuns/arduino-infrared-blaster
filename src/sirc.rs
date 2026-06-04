use arduino_hal::{
    delay_ns, delay_us,
    port::{Pin, PinOps, mode::Output},
};

const HALF_PERIOD_NS: u32 = 10_u32.pow(9) / (38_000 * 2);

const UNIT_BURST_LENGTH_US: u32 = 600;
const ZERO_BURST_LENGTH_US: u32 = UNIT_BURST_LENGTH_US;
const ONE_BURST_LENGTH_US: u32 = 2 * UNIT_BURST_LENGTH_US;
const START_BURST_LENGTH_US: u32 = 4 * UNIT_BURST_LENGTH_US;

pub fn send_space(pin: &mut Pin<Output, impl PinOps>) {
    pin.set_low();
    delay_us(ZERO_BURST_LENGTH_US);
}

pub fn send_one(pin: &mut Pin<Output, impl PinOps>) {
    const NUM_ITERATIONS: u32 = ONE_BURST_LENGTH_US * 1000 / HALF_PERIOD_NS;

    for _ in 0..NUM_ITERATIONS {
        pin.toggle();
        delay_ns(HALF_PERIOD_NS);
    }

    send_space(pin);
}

pub fn send_zero(pin: &mut Pin<Output, impl PinOps>) {
    const NUM_ITERATIONS: u32 = ZERO_BURST_LENGTH_US * 1000 / HALF_PERIOD_NS;

    for _ in 0..NUM_ITERATIONS {
        pin.toggle();
        delay_ns(HALF_PERIOD_NS);
    }

    send_space(pin);
}

pub fn send_start(pin: &mut Pin<Output, impl PinOps>) {
    const NUM_ITERATIONS: u32 = START_BURST_LENGTH_US * 1000 / HALF_PERIOD_NS;

    for _ in 0..NUM_ITERATIONS {
        pin.toggle();
        delay_ns(HALF_PERIOD_NS);
    }

    pin.set_low();
}

pub fn send_sirc_command(pin: &mut Pin<Output, impl PinOps>, address: u8, command: u8) {
    send_start(pin);

    send_space(pin);

    for i in 0..7 {
        let bit = (command >> i) & 1;
        if bit == 0 {
            send_zero(pin);
        } else {
            send_one(pin);
        }
    }

    for i in 0..5 {
        let bit = (address >> i) & 1;
        if bit == 0 {
            send_zero(pin);
        } else {
            send_one(pin);
        }
    }
}
