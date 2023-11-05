//! Pin control module.



use super::{
    common::*,
    gpio::GPIOControl,
    iocon::IOControl,
};


/// Anonymous pin type. This pin has all type and state information erased.
pub struct AnyPin {
    /// The port number of the pin.
    port: u8,

    /// The pin number of the pin.
    pin: u8,
}

impl AnyPin {
    /// Attempts to create the requested type pin from this anonymous pin.
    /// If the conversion fails it returns itself in the error.
    pub fn typed<const PORT: u8, const PIN: u8>(self) -> Result<Pin<PORT, PIN>, Self> {
        // Check if the port and pin match.
        if (PORT == self.port) && (PIN == self.pin) {
            // Forget this pin.
            core::mem::forget( self );

            // Create the typed pin.
            return Ok( Pin )
        }

        Err( self )
    }
}

impl<const PORT: u8, const PIN: u8> TryInto<Pin<PORT, PIN>> for AnyPin {
    type Error = Self;

    fn try_into(self) -> Result<Pin<PORT, PIN>, Self::Error> {
        self.typed()
    }
}

impl super::GPIOPin for AnyPin {
    fn port(&self) -> u32 {
        self.port as u32
    }

    fn pin(&self) -> u8 {
        self.pin
    }
}



/// Specific pin type.
pub struct Pin<const PORT: u8, const PIN: u8>;

impl<const PORT: u8, const PIN: u8> Pin<PORT, PIN> {
    /// Converts this typed pin into an anonymous pin.
    #[inline(always)]
    pub fn anon(self) -> AnyPin {
        // Forget this pin.
        core::mem::forget( self );

        // Create the anonymous pin.
        AnyPin { port: PORT, pin: PIN }
    }

    /// Creates the initial singleton.
    pub(crate) unsafe fn create() -> Self {
        Self
    }
}

impl<const PORT: u8, const PIN: u8> core::convert::Into<AnyPin> for Pin<PORT, PIN> {
    fn into(self) -> AnyPin {
        self.anon()
    }
}

impl<const PORT: u8, const PIN: u8> super::GPIOPin for Pin<PORT, PIN> {
    fn port(&self) -> u32 {
        PORT as u32
    }

    fn pin(&self) -> u8 {
        PIN
    }
}



/// Common trait for all GPIO pins, both typed and anonymous.
pub trait GPIOPin {
    /// Returns the port of this pin [0:1].
    fn port(&self) -> u32;

    /// Returns the number of this pin [0:31].
    fn pin(&self) -> u8;

    /// Sets the function of this pin.
    fn setfn(&mut self, function: u32) {
        IOControl::setfn( self.port(), self.pin(), function )
    }

    /// Sets the pull mode of this pin.
    fn setpull(&mut self, mode: PullMode) {
        IOControl::setpull( self.port(), self.pin(), mode )
    }

    /// Sets the slew rate of this pin.
    fn setspeed(&mut self, rate: SlewRate) {
        IOControl::setspeed( self.port(), self.pin(), rate )
    }

    /// Sets the input polarity of this pin.
    fn setpolarity(&mut self, polarity: Polarity) {
        IOControl::setpolarity( self.port(), self.pin(), polarity )
    }

    /// Sets the pin mode of this pin.
    fn setmode(&mut self, mode: GPIOMode) {
        IOControl::setmode( self.port(), self.pin(), mode )
    }

    /// Enables or disables the digital mode of this pin.
    fn digital(&mut self, d: bool) {
        IOControl::setdigital( self.port(), self.pin(), d )
    }
}
