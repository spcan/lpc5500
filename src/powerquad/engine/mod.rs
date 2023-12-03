//! PowerQuad AHB engine interface.





/// PowerQuad Coprocessor interface.
pub struct Engine;

impl Engine {
    /// Internal function to create a new instance.
    pub(super) fn create() -> Self {
        Self
    }

    /// Turns off the PowerQuad coprocessor (if all other PQ interfaces are off).
    pub fn sleep(self) -> SleepToken<N> {
        // Drop the interface to force a power off.
        core::mem::drop(self);

        // Create the sleep token.
        SleepToken::create()
    }
}

impl<const N: usize> Drop for Coprocessor<N> {
    fn drop(&mut self) {
        // Increase the POWEROFF counter to allow the other interfaces to sleep.
        super::poweroff();
    }
}



/// Sleep token to power on the PowerQuad.
pub struct SleepToken;

impl SleepToken {
    /// Internal function to create a new instance.
    pub(super) fn create() -> Self {
        Self
    }

    /// Turns on the PowerQuad.
    pub fn wake(self) -> Engine {
        // Power on the PowerQuad.
        super::poweron();

        // Create the sleep token.
        Engine::create()
    }
}
