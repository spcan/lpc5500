//! List of all DMA errors.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature="defmt", derive(defmt::Format))]
pub enum DMAError {
    /// Invalid count in the transfer. Count must be at least 1 and at most 1024.
    InvalidCount,
    /// Attempted to transfer from a buffer endpoint in an A2A transfer.
    NonAddressEndpoint,

    /// The source and destination regions of a transfer overlap.
    RegionOverlap,

    /// The source and target addresses have incompatible word sizes.
    WordSize,
}
