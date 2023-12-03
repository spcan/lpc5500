//! Interrupt module.



mod exceptions;
mod interrupts;



/// A vector for an interrupt or exception.
pub union Vector {
    /// Existing interrupt.
    ptr: u32,

    /// Reserved interrupt.
    res: u32,
}

impl Vector {
    /// Creates an interrupt.
    pub(self) const fn create(ptr: u32) -> Self {
        Self { ptr, }
    }

    /// Creates a reserved interrupt.
    pub(self) const fn reserved() -> Self {
        Self { res: 0, }
    }
}
