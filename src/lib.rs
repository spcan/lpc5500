
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
pub mod timers;

pub(crate) mod asm;

pub mod vtable;
mod peripherals;


mod macros;



#[cfg(feature = "defmt")]
mod defmt;



pub use peripherals::Peripherals;



#[link_section = ".RESETVECTOR"]
#[used]
static RESET: unsafe extern "C" fn() = Reset;


#[used]
pub static OSTIME: timers::ostimer::OSTime = unsafe { timers::ostimer::OSTime::uninit() };



/// Initializes the HAL and returns an instance of all peripherals.
#[no_mangle]
pub unsafe extern "C" fn Reset() {
    // Enable all memories.
    //mem::init();

    // Initialize the system.
    let user = system::init();

    // Initialize the vector tables and interrupts.
    vtable::init();

    // Initialize the DMAs.
    //dma::init();

    // Initialize the timers.
    let ostime = timers::init();

    // Initialize the pins.
    let pins = gpio::init();

    // Initialize other systems.
    security::init();

    //#[cfg(feature = "defmt")]
    //defmt::init();

    // Create the peripheral's singleton.
    let peripherals = Peripherals {
        pins,
        powerquad: powerquad::PowerQuad::create(),
        user,
        ostime,
        utick: timers::utick::MicroTick::create(),
    };

    // Call the external user function.
    extern "Rust" {
        static USERMAIN: fn(Peripherals) -> !;
    }

    USERMAIN( peripherals );
}
