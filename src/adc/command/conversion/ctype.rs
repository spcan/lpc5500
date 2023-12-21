//! Configures the conversion to be executed.



#[repr(Clone, Copy, Eq, PartialEq)]
pub enum ConversionType {
    /// Only Side A of the channel is measured.
    SideA,

    /// Only Side B of the channel is measured.
    SideB,

    /// Differential mode (Side A - Side B).
    Differential,

    /// Both Side A and Side B are measured.
    Both,
}
