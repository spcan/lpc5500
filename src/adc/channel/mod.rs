//! Hardware channels of the ADC.



mod advanced;
mod basic;



/// Instance of an ADC hardware channel.
pub struct Channel<const N: usize>;



/// The channel that measures the voltage supply of the ADC complex.
pub type ADCSupply = Channel<12>;

/// The channel that measures the 1 Volt bandgap in the ADC complex.
pub type ADC1V = Channel<13>;

/// The channel that measures the temperature sensor.
pub type Temperature = Channel<26>;



/// Common trait for all ADC channels.
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
