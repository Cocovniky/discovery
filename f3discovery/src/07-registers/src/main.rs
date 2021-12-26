#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM};

#[entry]
fn main() -> ! {
    let (mut itm, _) = aux7::init();

    // A magic address!
    const GPIOE_BSRR: u32 = 0x4800_1018;

    unsafe {
        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    unsafe {
        // Read from (write-only) GPIO BSRR register address
        let value = ptr::read_volatile(GPIOE_BSRR as *const u32);
        iprintln!(&mut itm.stim[0], "{:#x}: {:#x}", GPIOE_BSRR, value);

        // A bad address!
        const BAD: u32 = 0x48001800;

        // Read from bad address (no registers there)
        let value = ptr::read_volatile(BAD as *const u32);
        iprintln!(&mut itm.stim[0], "{:#x}: {:#x}", BAD, value);
    }
    loop {}
}

fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}
