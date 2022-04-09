#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, Peripherals};
use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
use cortex_m_rt::entry;

use stm32l5::stm32l552;
use rtt_target::{rtt_init_print, rprintln};

mod secure_rt_core;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // let mut peripherals = stm32l552::Peripherals::take().unwrap();
    // let gpioa = &peripherals.GPIOA;
    // gpioa.odr.modify(|_, w| w.odr0().set_bit());

    // let rng = peripherals.RNG;
    

    // let p = Peripherals::take().unwrap();
    // let mut sau = p.SAU;

    // sau.set_region(2, SauRegion{
    //     base_address: 0,
    //     limit_address: 1,
    //     attribute: SauRegionAttribute::NonSecureCallable,
    // }).unwrap();
    
    
    rtt_init_print!();
    rprintln!("hello world!");

    secure_rt_core::start();
    
    loop {
        // your code goes here
    }
}
