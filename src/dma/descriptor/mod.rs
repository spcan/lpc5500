//! DMA transfer descriptor module.



mod list;



pub(super) use list::{
    DMA0DESCRIPTORS, DMA1DESCRIPTORS,
};



#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Descriptor {
    /// Transfer configuration.
    pub(crate) config: u32,

    /// Last address of the source data.
    pub(crate) srcend: u32,

    /// Last address of the destination data.
    pub(crate) dstend: u32,

    /// Link to the next descriptor.
    pub(crate) next: u32,
}

impl Descriptor {
    /// Creates an uninitialized descriptor.
    pub const fn uninit() -> Self {
        Self { config: 0, srcend: 0, dstend: 0, next: 0, }
    }
}
