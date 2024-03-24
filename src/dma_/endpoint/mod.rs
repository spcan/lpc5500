//! DMA transfer endpoints.



mod address;
mod buffer;



pub use address::AddressEndpoint;
pub use buffer::BufferEndpoint;



/// Internal method for all endpoints.
pub(crate) trait Endpoint {
    /// Returns the lower and upper limit of the endpoint.
    fn bounds(&self) -> (u32, u32) {
        (self.start(), self.end())
    }

    /// Returns the end address.
    fn end(&self) -> u32;

    /// Returns the start address.
    fn start(&self) -> u32;

    /// Returns `true` if the endpoints overlap.
    fn overlaps<E: Endpoint>(&self, rhs: &E) -> bool {
        // Get the bounds of the endpoints.
        let (sa, ea) = self.bounds();
        let (sb, eb) = rhs.bounds();

        (sa <= eb) && (ea >= sb)
    }
}
