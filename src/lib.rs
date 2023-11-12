
#![no_std]

#![feature(generic_const_exprs)]


//mod critical;

pub mod gpio;
pub mod peripherals;

pub mod system;



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
            user,
        }
    }
}