
#![no_std]

#![allow(dead_code)]

#![feature(asm_const)]
#![feature(generic_const_exprs)]


//mod critical;

pub mod adc;
//pub mod dma;
pub mod gpio;
pub mod powerquad;
pub mod security;
pub mod system;

mod interrupts;
mod peripherals;



pub use peripherals::Peripherals;



/// Initializes the HAL and returns an instance of all peripherals.
pub unsafe fn init() -> Peripherals {
    // Initialize the system.
    let user = system::init();

    // Initialize the DMAs.
    //dma::init();

    // Initialize the timers.

    // Initialize the pins.
    let pins = gpio::init();

    // Initialize security system.
    security::init();

    Peripherals {
        pins,
        powerquad: powerquad::PowerQuad::create(),
        user,
    }
}
