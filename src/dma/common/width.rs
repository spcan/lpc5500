//! Indicates the width of each DMA transfer.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    /// Transfers are performed in bytes (8 bits).
    Byte = 0,

    /// Transfers are performed in half-words (16 bits).
    Half = 1,

    /// Transfers are performed in words (32 bits).
    Word = 2,
}

impl Width {
    /// Returns the size of the width in bytes.
    pub const fn bytes(&self) -> u32 {
        match self {
            Width::Byte => 1,
            Width::Half => 2,
            Width::Word => 4,
        }
    }
}
