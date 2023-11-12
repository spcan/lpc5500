
#![no_std]

#![feature(asm_const)]
#![feature(generic_const_exprs)]


//mod critical;

pub mod gpio;
pub mod peripherals;

pub mod system;

pub mod powerquad;



pub use peripherals::Peripherals;



/// Initializes the HAL and returns an instance of all peripherals.
pub unsafe fn init() -> Peripherals {
    // Initialize the system.
    let user = system::init();

    // Initialize the timers.

    // Initialize the pins.
    let pins = gpio::init();

    unsafe {
        Peripherals {
            pins,
            powerquad: powerquad::PowerQuad::create(),
            user,
        }
    }
}