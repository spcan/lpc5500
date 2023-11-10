
#![no_std]

#![feature(generic_const_exprs)]


//mod critical;

pub mod gpio;
pub mod peripherals;

mod system;



pub use peripherals::Peripherals;




/// Initializes the HAL and returns an instance of all peripherals.
pub unsafe fn init() -> Peripherals {
    // Initialize the system.
    system::init();

    unsafe {
        Peripherals {
            mainclk: system::clocks::main::MainClock::create(),
            pins: peripherals::Pins::create(),
        }
    }
}