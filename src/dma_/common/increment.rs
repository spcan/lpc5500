//! Indicates the increment applied to the address (source or destination) after each transfer.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Increment {
    /// The address is not incremented.
    None = 0,

    /// The address is incremented 1 x width.
    Wx1 = 1,

    /// The address is incremented 2 x width.
    Wx2 = 2,

    /// The address is incremented 3 x width.
    Wx4 = 3,
}
