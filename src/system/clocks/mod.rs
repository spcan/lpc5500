//! Clock control module.
//! NOTES :
//! 
//! ADC Clock max is 24 MHz
//! 
//! PLL1 reserved for main clock.



pub mod fro;
pub mod main;



/// Initializes the clocks of the device.
pub(crate) fn init() {
    #[cfg(feature = "defmt")]
    defmt::trace!("Powering up internal FROs...");

    // Ensure the FROs are running.
    fro::init();

    #[cfg(feature = "defmt")]
    defmt::trace!("FROs are enabled");

    // 
}
