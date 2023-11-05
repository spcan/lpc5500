//! Clock control module.



mod main;
mod osc;



/// Common trait for all clock signals.
pub trait ClockSignal {
    /// A token can be used to link this clock signal as input to other clock signals.
    type Token;

    /// Returns the frequency of the input clock signal.
    /// In the case of sources such as oscillators the input is zero.
    fn freqin(&self) -> u32;

    /// Returns the frequency of the output clock signal.
    fn freqout(&self) -> u32;

    /// Creates a token of this signal.
    fn token(&mut self) -> Self::Token;
}



/// Clock control.
/// Configures all the clock signals of the device.
pub struct ClockControl {
    /// Clock signals from internal and external oscillators.
    osc: osc::ClockControl,

    /// Clock control of the main clocks.
    main: main::ClockControl,
}



/// Common trait for all clock signals that hvae to be enabled to output a signal.
pub trait ClockEnable {
    /// Enables the clock signal.
    fn enable(&mut self);
}



/// Common trait for clock signals that can divide the input signal by a factor.
pub trait DivClock {
    /// The type of the divider. This type is used to impose limits on the
    /// values of the divider and avoid error handling.
    type Divider;

    /// Sets the divider of the clock signal.
    fn divide(&mut self, div: Self::Divider);
}



/// Initializes the clocks of the device.
pub(crate) fn init() {
    // Initialize the internal oscillators.
    //fro::Control::init();

    // Reset the System Clock to the 12 MHz FRO.

    // Disable all clocks to the peripherals.

    // Enable the external clock if provided.
    //#[cfg(feature = "xosc")]
    //xosc::init();
}