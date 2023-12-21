//! Implementation of an address endpoint.



use super::super::{
    DMAData,
    common::Width,
};



/// Information of a buffer endpoint.
pub struct AddressEndpoint {
    /// Base address.
    address: u32,

    /// The width of the element.
    width: Width,
}

impl AddressEndpoint {
    /// Creates a `BufferEndpoint` from the given buffer.
    pub fn create<D: DMAData>(address: &'static D) -> Self {
        Self {
            address: address as *const _ as u32,
            width: D::WIDTH,
        }
    }
}

impl super::Endpoint for AddressEndpoint {
    fn bounds(&self) -> (u32, u32) {
        ( self.address, self.address + self.width.bytes() )
    }

    fn end(&self) -> u32 {
        self.address
    }

    fn start(&self) -> u32 {
        self.address
    }
}
