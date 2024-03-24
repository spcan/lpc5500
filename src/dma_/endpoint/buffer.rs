//! Implementation of a buffer endpoint.



use super::super::{
    DMAData,
    common::Width,
};



/// Information of a buffer endpoint.
pub struct BufferEndpoint<'a> {
    /// Base address of the buffer.
    address: u32,

    /// The number of elements in the buffer.
    size: usize,

    /// The width of an element of the buffer.
    width: Width,

    #[doc(hidden)]
    _lifetime: core::marker::PhantomData<&'a ()>,
}

impl<'a> BufferEndpoint<'a> {
    /// Creates a `BufferEndpoint` from the given buffer.
    pub fn create<D: DMAData>(buffer: &'a [D]) -> Self {
        Self {
            address: buffer.as_ptr() as u32,
            size: buffer.len(),
            width: D::WIDTH,
            _lifetime: core::marker::PhantomData,
        }
    }
}

impl<'a> super::Endpoint for BufferEndpoint<'a> {
    fn end(&self) -> u32 {
        self.address + ( self.size as u32 * self.width.bytes() )
    }

    fn start(&self) -> u32 {
        self.address
    }
}
