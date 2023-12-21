//! Configures the conversion of an ADC command.



mod average;
mod ctype;
mod resolution;
mod sampletime;



pub use average::*;
pub use ctype::*;
pub use resolution::*;
pub use sampletime::*;



pub struct Conversion {
    /// Selects the number of samples that will be averaged.
    pub average: AverageCount,

    /// Selects the channel to convert (clamped to CH0 to CH31).
    pub channel: u8,

    /// The conversion to execute.
    pub ctype: ConversionType,

    /// Selects the resolution of the conversions.
    pub resolution: Resolution,

    /// Selects the sample time for the conversion.
    /// Higher sample time is required to measure high impedance signals.
    /// Higher sample time also reduces power consumption when high conversion rates are not needed.
    pub sampletime: SampleTime,
}

impl Conversion {
    /// Creates a `Conversion` with the given parameters.
    pub const fn create(average: AverageCount, ctype: ConversionType, resolution: Resolution, sampletime: SampleTime) -> Self {
        Self { average, channel: 0, ctype, resolution, sampletime, }
    }

    /// Creates the default `Conversion`.
    pub const fn new() -> Self {
        Self {
            average: AverageCount::None,
            channel: 0,
            ctype: ConversionType::SideA,
            resolution: Resolution::Standard,
            sampletime: SampleTime::Cycles4,
        }
    }
}
