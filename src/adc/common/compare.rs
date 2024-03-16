//! Compare options for advanced ADC channels.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Compare {
    /// Store the measurement only if the comparison is `true`.
    OneShot = 0b10,

    /// Repeats the conversion until the comparison is `true` then store the value.
    Repeat = 0b11,
}
