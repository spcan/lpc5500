//! Errors of the PowerQuad engine.




#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Floating Point Overflow error.
    Overflow,

    /// Not A Number error.
    Nan,

    /// Fixed Point Overflow error.
    FixedOverflow,

    /// Number Underflow error.
    Underflow,

    /// Bus Access error.
    Bus,
}
