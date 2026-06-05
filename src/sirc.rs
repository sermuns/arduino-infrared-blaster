use arduino_hal::{delay_ms, delay_us, pac::TC2};

use crate::{disable_carrier, enable_carrier};

fn send_one(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(1200);

    disable_carrier(tc2);
    delay_us(600);
}

fn send_zero(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(600);

    disable_carrier(tc2);
    delay_us(600);
}

fn send_start_burst(tc2: &TC2) {
    enable_carrier(tc2);
    delay_us(2400);

    disable_carrier(tc2);
    delay_us(600);
}

pub fn send_message(tc2: &TC2, address: u8, command: u8) {
    send_start_burst(tc2);

    // command
    for i in (0..7).rev() {
        if (command >> i) & 1 != 0 {
            send_one(tc2);
        } else {
            send_zero(tc2);
        }
    }

    // address
    for i in (0..5).rev() {
        if (address >> i) & 1 != 0 {
            send_one(tc2);
        } else {
            send_zero(tc2);
        }
    }
}
