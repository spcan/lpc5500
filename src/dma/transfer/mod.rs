//! DMA transfer control.



/// Transfer endpoint.
/// An endpoint can be either the source of a transfer or the destination.
pub enum Endpoint<'a> {
    /// Represents a peripheral endpoint.
    Peripheral(u32),

    /// Represents a buffer transfer.
    Buffer(&'a [u8]),
}
