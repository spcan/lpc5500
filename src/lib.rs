
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


mod macros;



pub use peripherals::Peripherals;



#[link_section = ".RESETVECTOR"]
#[used]
static RESET: unsafe extern "C" fn() -> ! = Reset;



/// Initializes the HAL and returns an instance of all peripherals.
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize the system.
    let user = system::init();

    // Initialize the DMAs.
    //dma::init();

    // Initialize the timers.

    // Initialize the pins.
    let pins = gpio::init();

    // Initialize security system.
    security::init();

    let peripherals = Peripherals {
        pins,
        powerquad: powerquad::PowerQuad::create(),
        user,
    };

    // Call the external user function.
    extern "Rust" {
        static USERMAIN: fn(Peripherals) -> !;
    }

    USERMAIN( peripherals );
}
