//! DMA descriptor module.
//! Descriptors contain the configuration of DMA transfers.



use super::config::*;



/// DMA descriptor structure.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Descriptor<const N: usize> {
    /// Transfer configuration.
    pub(self) config: u32,

    /// Last source address of the DMA transfer.
    pub(self) srcend: u32,

    /// Last destination address of the DMA transfer.
    pub(self) dstend: u32,

    /// Address of the nex
    /// t DMA descriptor in the chain.
    pub(self) next: u32,
} 

impl<const N: usize> Descriptor<N> {
    /// Static initializer.
    pub(self) const fn empty() -> Self {
        Self { config: 0, srcend: 0, dstend: 0, next: 0, }
    }

    /// Links this descriptor to the given descriptor.
    pub fn link(&mut self, next: &Self) {
        self.next = next as *const _ as u32;
    }
}



/// Wrapper over an address to address transfer.
#[repr(transparent)]
pub struct A2ATransfer<const N: usize>(Descriptor<N>);

impl<const N: usize> A2ATransfer<N> {
    /// Returns a reference to the inner `Descriptor`.
    #[inline(always)]
    pub fn inner(&mut self) -> &mut Descriptor<N> {
        &mut self.0
    }

    /// Configures this descriptor's transfer.
    pub fn transfer(&mut self, src: DMAEndpoint, dst: DMAEndpoint, count: usize) -> Result<(), DMAError> {
        // Validate the endpoints.
        if src.overlaps(&dst) { return Err( DMAError::RegionOverlap ); }

        // Validate the number count.
        if (count > 1024) | (count < 1) { return Err( DMAError::InvalidCount ); }

        // Unpack the addresses and widths of the endpoints.
        match (src, dst) {
            (DMAEndpoint::Address(srcend, srcw), DMAEndpoint::Address(dstend, dstw)) => {
                // Validate the width of the endpoints.
                if srcw != dstw { return Err( DMAError::WordSize ); }

                // Set the addresses in the configuration.
                self.0.srcend = srcend;
                self.0.dstend = dstend;

                // Clear the configuration.
                self.0.config &= !((0x3FF << 16) | (0x3 << 14) | (0x3 << 12) | (0x3 << 8));

                // Set the width of the transfer and the count.
                self.0.config |= (((count - 1) as u32) << 16) | ((srcw as u32) << 8);

                Ok(())
            },

            _ => Err( DMAError::NonAddressEndpoint ),
        }
    }
}





/// Describes a DMA transfer endpoint (source or destination).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DMAEndpoint {
    /// Describes a buffer.
    Buffer( u32, usize, Width ),

    /// Describes a single address.
    Address( u32, Width ),
}

impl DMAEndpoint {
    /// Gets the bounds (upper exclusive) of the endpoint.
    pub fn bounds(&self) -> (u32, u32) {
        match self {
            DMAEndpoint::Address( address, width ) => (*address, address + width.bytes()),
            DMAEndpoint::Buffer( address, size, width ) => (*address, address + ( width.bytes() * (*size as u32) )),
        }
    }

    /// Validates the two endpoints and unpacks the configuration data.
    pub const fn validates(&self, rhs: &Self) -> Result<(Increment, Increment), DMAError> {
        // Check if the two regions overlap.
        if self.overlaps(rhs) { return Err( DMAError::RegionOverlap ); }

        // Check size compatibility.
        match self {
            DMAEndpoint::Address( _, x ) => match rhs {
                // Ensure both addresses have the same width.
                DMAEndpoint::Address( _, y ) => if *x == *y {
                    Ok( ( Increment::None, Increment::None ) )
                } else {
                    Err( DMAError::WordSize )
                },

                // Check if both widths are the same.
                DMAEndpoint::Buffer( _, size, y ) => if *x == *y {
                    Ok( ( Increment::None, x.relative(y) ) )
                },
            },
        };

        if !sizecompat { return Err( DMAError::WordSizeIncompatible ); }
    }

    /// Checks if two DMA endpoints overlap.
    const fn overlaps(&self, rhs: &Self) -> bool {
        // Get the start and end address of self and rhs.
        let (sa, ea) = self.bounds();
        let (sb, eb) = rhs.bounds();

        (sa <= eb) && (ea >= sb)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DMAEndpoint {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            DMAEndpoint::Address( address, width ) => defmt::write!(fmt, "{} @ {=u32:#X}", width, address ),
            DMAEndpoint::Buffer( address, size, width ) => defmt::write!(fmt, "[{}; {=usize}] @ {=u32:#X}", width, size, address),
        }
    }
}



