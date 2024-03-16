//! Bit resolution of an ADC conversion.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    /// Standard resolution.
    ///  - Channel: 12-bit resolution.
    ///  - Differential: 13-bit resolution (2's complement).
    Standard,

    /// High resolution.
    ///  - Channel: 16-bit resolution.
    ///  - Differential: 16-bit resolution (2's complement).
    High,
}
