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


use crate::system::SystemControl;



/// Internal GPIO abstraction to allow system control.
pub(crate) struct GPIOPort<const N: u8>;

impl<const N: u8> crate::system::Control for GPIOPort<N> {
    const REG: u32 = 0;
    const OFF: u8 = 14 + N;
}

impl<const N: u8> crate::system::enable::Enable for GPIOPort<N> {}

impl<const N: u8> crate::system::reset::Unreset for GPIOPort<N> {}



/// Initializes the GPIO pins.
pub(super) fn init() -> Pins {
    // Unreset GPIO ports 0 and 1.
    SystemControl::unreset::<GPIOPort<0>>();
    SystemControl::unreset::<GPIOPort<1>>();

    // Enable GPIO ports 0, 1, 2 and 3.
    SystemControl::enable::<GPIOPort<0>>();
    SystemControl::enable::<GPIOPort<1>>();
    SystemControl::enable::<GPIOPort<2>>();
    SystemControl::enable::<GPIOPort<3>>();

    // Unreset and enable IOControl.
    SystemControl::unreset::<IOControl>();
    SystemControl::enable::<IOControl>();

    Pins {
        PORT0: unsafe { Port::create() },
        PORT1: unsafe { Port::create() },
    }
}



/// Collection of all pins in the device.
/// Pins are divided into ports, to get a pin (e.g. PIN_0_0)
/// it would be Pins.PORT0.PIN0.
#[allow(non_snake_case)]
pub struct Pins {
    /// GPIO port 0.
    pub PORT0: Port<0>,

    /// GPIO port 1.
    pub PORT1: Port<1>,
}



/// A port is a group of pins.
#[allow(non_snake_case)]
pub struct Port<const N: u8> {
    pub PIN0 : Pin<N,  0>,
    pub PIN1 : Pin<N,  1>,
    pub PIN2 : Pin<N,  2>,
    pub PIN3 : Pin<N,  3>,
    pub PIN4 : Pin<N,  4>,
    pub PIN5 : Pin<N,  5>,
    pub PIN6 : Pin<N,  6>,
    pub PIN7 : Pin<N,  7>,
    pub PIN8 : Pin<N,  8>,
    pub PIN9 : Pin<N,  9>,

    pub PIN10: Pin<N, 10>,
    pub PIN11: Pin<N, 11>,
    pub PIN12: Pin<N, 12>,
    pub PIN13: Pin<N, 13>,
    pub PIN14: Pin<N, 14>,
    pub PIN15: Pin<N, 15>,
    pub PIN16: Pin<N, 16>,
    pub PIN17: Pin<N, 17>,
    pub PIN18: Pin<N, 18>,
    pub PIN19: Pin<N, 19>,

    pub PIN20: Pin<N, 20>,
    pub PIN21: Pin<N, 21>,
    pub PIN22: Pin<N, 22>,
    pub PIN23: Pin<N, 23>,
    pub PIN24: Pin<N, 24>,
    pub PIN25: Pin<N, 25>,
    pub PIN26: Pin<N, 26>,
    pub PIN27: Pin<N, 27>,
    pub PIN28: Pin<N, 28>,
    pub PIN29: Pin<N, 29>,

    pub PIN30: Pin<N, 30>,
    pub PIN31: Pin<N, 31>,
}

impl<const N: u8> Port<N> {
    /// Creates the initial singleton.
    pub(self) unsafe fn create() -> Self {
        Self {
            PIN0 : Pin::create(),
            PIN1 : Pin::create(),
            PIN2 : Pin::create(),
            PIN3 : Pin::create(),
            PIN4 : Pin::create(),
            PIN5 : Pin::create(),
            PIN6 : Pin::create(),
            PIN7 : Pin::create(),
            PIN8 : Pin::create(),
            PIN9 : Pin::create(),
        
            PIN10: Pin::create(),
            PIN11: Pin::create(),
            PIN12: Pin::create(),
            PIN13: Pin::create(),
            PIN14: Pin::create(),
            PIN15: Pin::create(),
            PIN16: Pin::create(),
            PIN17: Pin::create(),
            PIN18: Pin::create(),
            PIN19: Pin::create(),
        
            PIN20: Pin::create(),
            PIN21: Pin::create(),
            PIN22: Pin::create(),
            PIN23: Pin::create(),
            PIN24: Pin::create(),
            PIN25: Pin::create(),
            PIN26: Pin::create(),
            PIN27: Pin::create(),
            PIN28: Pin::create(),
            PIN29: Pin::create(),
        
            PIN30: Pin::create(),
            PIN31: Pin::create(),
        }
    }
}
