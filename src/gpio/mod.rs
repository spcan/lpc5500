//! General Purpose I/O module.



pub(self) mod gpio;
pub(self) mod iocon;

pub mod common;

mod rawio;
mod pin;



pub(crate) use gpio::GPIOControl;
pub(crate) use iocon::IOControl;
pub(crate) use pin::GPIOPin;

pub use pin::{
    AnyPin, Pin,
};

pub use rawio::{
    /*Input, */Output,
};



/// Internal GPIO abstraction to allow system control.
pub(crate) struct GPIOPort<const N: u8>;

impl<const N: u8> crate::system::Control for GPIOPort<N> {
    const REG: u32 = 0;
    const OFF: u8 = 14 + N;
}

impl<const N: u8> crate::system::enable::Enable for GPIOPort<N> {}

impl<const N: u8> crate::system::reset::Unreset for GPIOPort<N> {}
