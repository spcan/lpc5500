//! PowerQuad handler container.
//! Contains all informationfor the asynchronous handler.



pub(crate) struct Container {
    /// Marks the end of the operation.
    pub(crate) end: bool,

    /// Contains the error (if any) that ocurred during operation.
    pub(crate) error: u32,
}

impl Container {
    /// Static initializer.
    pub(super) const fn empty() -> Self {
        Container { end: false, error: 0, }
    }
}
