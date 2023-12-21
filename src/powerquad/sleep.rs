//! Internal utility to power off the coprocessor when idle.



pub struct SleepToken<T: Sleep>(core::marker::PhantomData<T>);

impl<T: Sleep> SleepToken<T> {
    /// Internal function to create a new instance.
    pub(super) fn create() -> Self {
        Self(core::marker::PhantomData)
    }

    /// Turns on the PowerQuad.
    pub fn wake(self) -> T {
        // Power on the PowerQuad.
        super::poweron();

        // Recreate the waking instance.
        T::create()
    }
}



/// Internal trait for the PowerQuad components.
pub trait Sleep: Sized {
    /// Internal method to create an instance of the component.
    fn create() -> Self;

    /// Internal method to set the component to sleep.
    fn sleep(self) -> SleepToken<Self> {
        // Drop the component.
        core::mem::drop(self);

        // Create the token.
        SleepToken::<Self>::create()
    }
}
