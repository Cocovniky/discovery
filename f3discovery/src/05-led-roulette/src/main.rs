#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut tick_period = 64_u8;
    let v_tick_period = volatile::Volatile::new(&mut tick_period);

    let leds_cnt = leds.len();
    loop {
        for i in (leds_cnt - 2)..(leds_cnt - 2 + leds_cnt) {
            leds[i % leds_cnt].off().ok();
            delay.delay_ms(v_tick_period.read());
            leds[(i + 2) % leds_cnt].on().ok();
            delay.delay_ms(v_tick_period.read());
        }
    }
}
