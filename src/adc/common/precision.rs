//! Configuration of the precision of an ADC measurement.
//! Encompasses the sample time and sample average of a conversion.



use super::*;



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Precision {
    /// Number of cycles to measure for a sample.
    sampletime: SampleTime,

    /// Number of samples to average for a conversion.
    average: Option<Average>,
}

impl Precision {
    /// Creates the precision configuration for the fastest conversions but low precision.
    const FASTEST : Self = Self { sampletime: SampleTime::Cycles4,   average: None,                        };

    /// Creates the precision configuration for fast conversions and medium precision.
    const FAST    : Self = Self { sampletime: SampleTime::Cycles8,   average: Some( Average::Samples4   ), };

    /// Creates a precision configuration balanced between speed and precision.
    const BALANCED: Self = Self { sampletime: SampleTime::Cycles20,  average: Some( Average::Samples16  ), };

    /// Creates a high precision but slow configuration.
    const HIGH    : Self = Self { sampletime: SampleTime::Cycles36,  average: Some( Average::Samples32  ), };

    /// Creates the highest precision configuration at the expense of speed.
    const MAXIMUM : Self = Self { sampletime: SampleTime::Cycles132, average: Some( Average::Samples128 ), };
}
