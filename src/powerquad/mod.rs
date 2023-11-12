//! PowerQuad Coprocessor module.



pub mod coprocessor;



use coprocessor::Coprocessor;

use crate::system::{
    Control, SystemControl,
    enable::{
        Disable, Enable,
    },
    reset::{
        Reset, Unreset,
    },
};



pub struct PowerQuad;

impl PowerQuad {
    /// Internal function to create an instance.
    pub(crate) fn create() -> Self {
        Self
    }

    /// Initializes the PowerQuad.
    /// Returns all the interfaces to the PowerQuad.
    pub fn init(self) -> (Coprocessor<0>, Coprocessor<1>) {
        // Reset the peripheral.
        SystemControl::reset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        // Unreset the peripheral.
        SystemControl::unreset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        (Coprocessor::create(), Coprocessor::create())
    }
}

impl Control for PowerQuad {
    const REG: u32 = 2;
    const OFF: u8 = 19;
}

impl Enable  for PowerQuad {}
impl Unreset for PowerQuad {}

unsafe impl Disable for PowerQuad {}
unsafe impl Reset   for PowerQuad {}
