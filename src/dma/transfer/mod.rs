//! DMA transfer module.



mod b2b;



pub use b2b::B2BTransfer;



/// Common trait for all transfers.
pub trait Transfer {
    /// Returns the destination end address.
    fn dstend(&self) -> u32;

    /// Returns the source end address.
    fn srcend(&self) -> u32;
}
