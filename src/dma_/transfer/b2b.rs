//! Configuration of a buffer to buffer transfer.



use super::super::{
    DMAData,
    endpoint::{
        BufferEndpoint, Endpoint,
    },
    error::DMAError,
};



pub struct B2BTransfer<'a, 'b> {
    /// Source buffer.
    src: BufferEndpoint<'a>,

    /// Destination buffer.
    dst: BufferEndpoint<'b>,
}

impl<'a, 'b> B2BTransfer<'a, 'b> {
    /// Creates a `B2BTransfer` configuration.
    pub fn create<A: DMAData, B: DMAData>(src: &'a [A], dst: &'b mut [B]) -> Result<Self, DMAError> {
        // Create both endpoints.
        let src = BufferEndpoint::create( src );
        let dst = BufferEndpoint::create( dst );

        // Validate that they do not overlap.
        if src.overlaps(&dst) { return Err( DMAError::RegionOverlap ) }

        Ok( Self { src, dst, } )
    }
}

impl<'a, 'b> super::Transfer for B2BTransfer<'a, 'b> {
    fn dstend(&self) -> u32 {
        self.dst.end()
    }

    fn srcend(&self) -> u32 {
        self.src.end()
    }
}
