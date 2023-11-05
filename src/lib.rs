
#![no_std]



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
            pins: peripherals::Pins::create(),
        }
    }
}