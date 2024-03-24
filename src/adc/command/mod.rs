//! ADC Commands.



pub struct Command<const C: usize> {
    /// Command low word.
    pub(super) lo: u32,

    /// Command high word.
    pub(super) hi: u32,
}

impl<const C: usize> CommandConfig for Command<C> {

}



/// Common trait for all commands.
pub trait CommandConfig {
    /// Selects the channel to read from.
    fn channel<const N: usize>(&mut self, channel: Channel<N>) {
        
    }
}
