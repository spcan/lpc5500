//! DMA Channel control.
//! Controls the sequence of transfers performed by a DMA Channel.



use super::{
    descriptor::{
        Descriptor, DescriptorHandle,
    },
    transfer::Transfer,
};



pub struct DMAChannel {
    dma: usize,
    channel: usize,
}

impl DMAChannel {
    /// Sets the first transfer of the DMA Channel.
    pub fn begin<'a, T: Transfer>(&mut self, transfer: &T) -> SequenceHandle<'a> {
        // Store the transfer in the first descriptor.
        let handle = self.first().store( transfer );

        // Get a reference to the first descriptor and copy its configuration.
        let config = handle.reference().config;

        // 
    }

    /// Returns a reference to the first descriptor.
    fn first(&mut self) -> &mut Descriptor {
        match self.dma {
            0 => unsafe { &mut super::dma0::DMA0DESCRIPTORS[self.channel] },
            _ => unsafe { &mut super::dma1::DMA1DESCRIPTORS[self.channel] },
        }
    }
}



/// The handle of a sequence of descriptors for a DMA channel.
pub struct SequenceHandle<'a> {
    /// The DMA channel that owns this sequence.
    channel: &'a DMAChannel,

    /// The last descriptor of the sequence.
    last: &'a mut Descriptor<'a>,
}

impl<'a> SequenceHandle<'a> {
    /// Starts the transfer sequence.
    pub fn start(&mut self) {

    }

    /// Links the given descriptor to the sequence.
    pub fn link(&mut self, mut descriptor: DescriptorHandle<'a>) {
        // Store the next descriptor in the current las descriptor.
        self.last.next = Some( descriptor.reference() );

        // Swap the last descriptor reference.
        self.last = descriptor.reference();

        // Forget the descriptor handle.
        core::mem::forget(descriptor);
    }
}
