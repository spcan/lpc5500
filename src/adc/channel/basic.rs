//! Basic ADC channel usage.
//! All channels except the temperature channel implement this functionality.



use super::{
    ADCChannel, Channel,

    super::{
        Command,

        common::*,
    },
};



/// Common trait for the basic functionality of an ADC channel.
pub trait ADCBasic: ADCChannel {
    /// Creates a command to read the channel's side A.
    fn read(&mut self, resolution: Resolution, precision: Precision) -> Command {
        Command::create( Self::NUMBER, Mode::SideA, resolution, precision)
    }
}



impl ADCBasic for Channel<0> {}
impl ADCBasic for Channel<1> {}
impl ADCBasic for Channel<2> {}
impl ADCBasic for Channel<3> {}
impl ADCBasic for Channel<4> {}

impl ADCBasic for Channel<12> {}
impl ADCBasic for Channel<13> {}
