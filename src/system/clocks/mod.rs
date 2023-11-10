//! Clock control module.
//! NOTES :
//! 
//! ADC Clock max is 24 MHz
//! 
//! PLL1 reserved for main clock.



pub mod main;

pub mod fro;



/// Initializes the clocks of the device.
pub(crate) fn init() {
    // Ensure the FROs are running.
    fro::Control::init();

    // 
}



/// Common trait for all clock signals.
pub trait ClockTrait {
    /// A token representing this clock.
    type Token;

    /// Returns the current frequency of the clock in Hertz.
    fn frequency(&self) -> u32;

    /// Creates a token of the clock to use as source for other clocks.
    fn token(&mut self) -> Self::Token;
}

