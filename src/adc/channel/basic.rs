//! Basic ADC channel usage.



use super::ADCChannel;



pub trait ADCBasic: ADCChannel {
    /// Creates a command to read the channel's side A.
    fn read(&mut self) {
        
    }
}
