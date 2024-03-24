//! A handle to a descriptor that contains a transfer configuration.



use super::Descriptor;



pub struct DescriptorHandle<'a> {
    /// The descriptor that created this handle.
    descriptor: &'a mut Descriptor<'a>,
}

impl<'a> DescriptorHandle<'a> {
    /// Creates a descriptor handle.
    pub(super) fn create(descriptor: &'a mut Descriptor) -> Self {
        Self { descriptor, }
    }

    /// Returns a reference to the descriptor.
    pub(crate) fn reference(&mut self) -> &mut Descriptor {
        self.descriptor
    }
}
