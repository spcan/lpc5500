//! A list of descriptors with static initialization.



use super::Descriptor;



#[derive(Clone, Copy)]
#[repr(C, align(512))]
pub struct DescriptorList<'a, const N: usize>(pub(super) [Descriptor<'a>; N]);

impl<'a, const N: usize> DescriptorList<'a, N> {
    /// Creates an empty descriptor list.
    pub const fn empty() -> Self {
        Self([Descriptor::empty(); N])
    }
}

impl<'a, const N: usize> core::ops::Index<usize> for DescriptorList<'a, N> {
    type Output = Descriptor<'a>;

    fn index(&self, index: usize) -> &Descriptor<'a> {
        &self.0[index]
    }
}

impl<'a, const N: usize> core::ops::IndexMut<usize> for DescriptorList<'a, N> {
    fn index_mut(&mut self, index: usize) -> &mut Descriptor<'a> {
        &mut self.0[index]
    }
}
