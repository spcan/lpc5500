//! Selects the amount of ADC clocks spent on each conversion.



#[repr(Clone, Copy, Eq, PartialEq)]
pub enum SampleTime {
    /// Sample time of 4 (3.5) cycles of ADC clock.
    Cycles4,

    /// Sample time of 6 (5.5) cycles of ADC clock.
    Cycles6,

    /// Sample time of 8 (7.5) cycles of ADC clock.
    Cycles8,

    /// Sample time of 12 (11.5) cycles of ADC clock.
    Cycles12,

    /// Smaple time of 20 (19.5) cycles of ADC clock.
    Cycles20,

    /// Smaple time of 36 (35.5) cycles of ADC clock.
    Cycles36,

    /// Smaple time of 68 (67.5) cycles of ADC clock.
    Cycles68,

    /// Smaple time of 132 (131.5) cycles of ADC clock.
    Cycles132,
}