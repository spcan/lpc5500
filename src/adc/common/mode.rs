//! Modes of an ADC conversion.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Only Side A of the channel is measured.
    SideA,

    /// Only Side B of the channel is measured.
    SideB,

    /// Differential mode (Side A - Side B).
    Diff,

    /// Both Side A and Side B are measured.
    Both,
}
