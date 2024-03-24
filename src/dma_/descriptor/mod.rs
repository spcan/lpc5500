//! DMA transfer descriptor.



mod handle;
mod list;



pub use handle::DescriptorHandle;
pub use list::DescriptorList;

use super::transfer::Transfer;



/// DMA descriptor raw structure.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Descriptor<'a> {
    /// Transfer configuration.
    pub(super) config: u32,

    /// Last source address of the DMA transfer.
    pub(super) srcend: u32,

    /// Last destination address of the DMA transfer.
    pub(super) dstend: u32,

    /// Address of the next DMA descriptor in the chain.
    pub(super) next: Option<&'a Self>,
} 

impl<'a> Descriptor<'a> {
    /// Static initializer.
    pub(self) const fn empty() -> Self {
        Self { config: 0, srcend: 0, dstend: 0, next: None, }
    }

    /// Stores a transfer in this descriptor.
    pub fn store<T: Transfer>(&mut self, transfer: &T) -> DescriptorHandle<'a> {
        // Store the source and destination addresses in the descriptor.
        self.srcend = transfer.srcend();
        self.dstend = transfer.dstend();

        // Set up the configuration.
        self.config = transfer.config();

        // Create the handle and return it.
        DescriptorHandle::create( self )
    }
}
