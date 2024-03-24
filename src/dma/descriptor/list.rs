//! DMA 0 and DMA 1 descriptor lists.



/// Descriptor list of DMA 0.
//#[link_section = ".bss"]
#[used]
pub(crate) static DMA0DESCRIPTORS: DescriptorList<23> = DescriptorList::uninit();


/// Descriptor list of DMA 1.
//#[link_section = ".bss"]
#[used]
pub(crate) static DMA1DESCRIPTORS: DescriptorList<10> = DescriptorList::uninit();



/// N sized descriptor list.
#[repr(align(512))]
pub(crate) struct DescriptorList<const N: usize>([super::Descriptor; N]);

impl<const N: usize> DescriptorList<N> {
    /// Creates an uninitialized descriptor list.
    pub(self) const fn uninit() -> Self {
        Self([super::Descriptor::uninit(); N])
    }
}
