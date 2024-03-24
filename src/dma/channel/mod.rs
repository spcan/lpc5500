//! DMA Channel control.



/// Instance of a channel.
pub struct Channel<const N: usize, const C: usize>;

impl<const N: usize, const C: usize> Channel<N, C> {
    /// Creates a singleton of the `Channel`.
    pub(super) fn create() -> Self {
        Self
    }

    /// Enables or disables the hardware trigger for the DMA channel.
    fn hwtrigger(&mut self, enable: bool) {
        // Get the config register.
//        let register = self.configreg();


    }
}
