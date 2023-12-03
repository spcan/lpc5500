//! PowerQuad Coprocessor module.



pub mod coprocessor;
pub mod engine;



use core::sync::atomic::{
    AtomicU32, Ordering,
};

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

use engine::MathEngine;



#[link_section = ".data.LPC5500.powerquad.POWEROFF"]
pub(self) static POWER: AtomicU32 = AtomicU32::new( 0 );



pub struct PowerQuad;

impl PowerQuad {
    /// Internal function to create an instance.
    pub(crate) fn create() -> Self {
        Self
    }

    /// Initializes the PowerQuad.
    /// Returns all the interfaces to the PowerQuad.
    pub fn init(self) -> (Coprocessor<0>, Coprocessor<1>, MathEngine) {
        // Reset the peripheral.
        SystemControl::reset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        // Unreset the peripheral.
        SystemControl::unreset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        (Coprocessor::create(), Coprocessor::create(), MathEngine::create())
    }

    /// Powers off the PowerQuad.
    #[inline(always)]
    fn poweroff() {
        // Disable the clock to the power quad.
        SystemControl::disable::<Self>();
    }

    /// Powers on the PowerQuad.
    #[inline(always)]
    fn poweron() {
        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();
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



/// Internal function to turn on the PowerQuad coprocessor.
pub(self) fn poweron() {
    // Decrement the current value and read the last value.
    let _ = POWER.fetch_sub(1, Ordering::AcqRel);

    // Power on.
    PowerQuad::poweron();
}

/// Internal function to request to turn off the PowerQuad coprocessor.
pub(self) fn poweroff() {
    // Increment the current value and read the last value.
    let last = POWER.fetch_add(1, Ordering::AcqRel);

    // Power off if the other two devices have requested a power off.
    if last >= 2 { PowerQuad::poweroff(); }
}
