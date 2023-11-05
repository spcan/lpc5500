//! Raw I/O control.


use super::{
    GPIOPin,
    gpio::GPIOControl,
};


/// Wraps a GPIO pin and enables the raw output functions.
pub struct Output<P: GPIOPin>(P);

impl<P: GPIOPin> Output<P> {
    /// Creates a new `Output`.
    pub fn new(mut pin: P) -> Self {
        // Set the direction of the GPIO pin.
        GPIOControl::output( pin.port(), pin.pin() );

        // Set the function of the pin to raw output.
        pin.setfn( 0 );

        Self(pin)
    }

    /// Sets the pin output.
    pub fn set(&mut self) {
        GPIOControl::set( self.0.port(), self.0.pin())
    }

    /// Clears the pin output.
    pub fn clear(&mut self) {
        GPIOControl::clear( self.0.port(), self.0.pin())
    }

    /// Toggles the pin output.
    pub fn toggle(&mut self) {
        GPIOControl::toggle( self.0.port(), self.0.pin())
    }
}
