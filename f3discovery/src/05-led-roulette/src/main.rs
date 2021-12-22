#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 128_u16;
    let v_half_period = volatile::Volatile::new(&mut half_period);

    loop {
        for i in 0..8 {
            leds[i].on().ok();
            delay.delay_ms(v_half_period.read());
        }
        for i in 0..8 {
            leds[i].off().ok();
            delay.delay_ms(v_half_period.read());
        }
    }
}
