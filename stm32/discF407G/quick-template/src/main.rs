#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
// use panic_halt as _; // panic handler

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Disc-4207G it's connected to pin PD12 to PD15.
        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd12.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            // led.toggle();
            rprintln!("Yay I am on!!");
            led.set_high();
            delay.delay_ms(3000_u32);

            rprintln!("Oh man I am off!!");
            led.set_low();
            delay.delay_ms(3000_u32);

        }
    }

    loop {}
}