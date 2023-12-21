//! Configures the bit resolution of a conversion.



#[repr(Clone, Copy, Eq, PartialEq)]
pub enum Resolution {
    /// 12 bit resolution for single conversions.
    /// 13 bit (2s complement) resolution for differential conversions.
    Standard,

    /// 16 bit resolution for single conversions.
    /// 16 bit (2s complement) resolution for differential conversions.
    High,
}
