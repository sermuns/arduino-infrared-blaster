use arduino_hal::{delay_us, pac::TC2};

use crate::{disable_carrier, enable_carrier};

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

pub fn send_message(tc2: &TC2, address: u16, command: u8) {
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
