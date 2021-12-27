#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Clear the update event flag
    //tim6.sr.modify(|_, w| w.uif().clear_bit());
    tim6.sr.write(|w| w.uif().clear_bit());

    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN: Enable the counter
    //tim6.cr1.modify(|_, w| w.cen().set_bit());
    tim6.cr1.write(|w| w.cen().set_bit());

    // Busy-wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Power on the TIM6 timer
    //rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    rcc.apb1enr.write(|w| w.tim6en().set_bit());
    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // Configure the prescaler to have the counter operate at 1 KHz
    // frequency = apb1  / (psc   + 1), apb1 = 8MHz
    // 1 KHz     = 8 MHz / (7_999 + 1)
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 64;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
