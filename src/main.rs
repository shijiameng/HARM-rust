#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use alloc_cortex_m::CortexMHeap;

use cortex_m::{asm, Peripherals};
use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
use cortex_m_rt::{entry, exception, pre_init, ExceptionFrame};
use core::mem::MaybeUninit;
use core::alloc::Layout;
use core::panic::PanicInfo;
use lpc55_hal as hal;

use rtt_target::{rtt_init_print, rprintln};

mod secure_rt_core;

extern "C" {
    fn BOARD_Init();
    fn BOARD_EnableSysTick();
    fn BOARD_InitBootTEE();
    fn BOARD_InitMPU();
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024;
static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    unsafe { 
        ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE); 
        BOARD_Init();
    }
    
    rtt_init_print!();

    rprintln!("hello world");
    
    secure_rt_core::start(0x2001a000, 0x2a00);
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    rprintln!("[ALLOCATOR] !!! Out of memory !!!");
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    rprintln!("!!! Panic !!!");
    loop {}
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    rprintln!("!!! Hard fault !!!");
    loop {}
}

#[pre_init]
unsafe fn before_main() {
    BOARD_InitBootTEE();
    BOARD_InitMPU();
}