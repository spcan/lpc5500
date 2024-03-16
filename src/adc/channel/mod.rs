//! Channels of the ADC.
//! Each channel can generate commands that must be then registered with the ADC
//! to be executed.



mod advanced;
mod basic;



pub use advanced::ADCAdvanced;
pub use basic::ADCBasic;



/// Instance of an ADC Channel.
pub struct Channel<const N: usize>;


/// The channel that measures the supply voltage of the ADC complex.
pub type ADCSupply = Channel<12>;

/// The channel that measures the 1 Volt bandgap in the ADC complex.
pub type ADC1V = Channel<13>;

/// The channel that measure the temperature sensor.
pub type Temperature = Channel<26>;



/// Common trait for all ADC Channels.
pub trait ADCChannel {
    const NUMBER: u8;
}



impl ADCChannel for Channel<0> {
    const NUMBER: u8 = 0;
}

impl ADCChannel for Channel<1> {
    const NUMBER: u8 = 1;
}

impl ADCChannel for Channel<2> {
    const NUMBER: u8 = 2;
}

impl ADCChannel for Channel<3> {
    const NUMBER: u8 = 3;
}

impl ADCChannel for Channel<4> {
    const NUMBER: u8 = 4;
}

impl ADCChannel for Channel<12> {
    const NUMBER: u8 = 12;
}

impl ADCChannel for Channel<13> {
    const NUMBER: u8 = 13;
}

impl ADCChannel for Channel<26> {
    const NUMBER: u8 = 26;
}
