//! Interrupt module.



mod exceptions;
mod interrupts;



/// A vector for an interrupt or exception.
pub union Vector {
    /// Existing interrupt.
    ptr: unsafe extern "C" fn(),

    /// Reserved interrupt.
    res: u32,
}

impl Vector {
    /// Creates an interrupt.
    pub(self) const fn create(ptr: unsafe extern "C" fn()) -> Self {
        Self { ptr, }
    }

    /// Creates a reserved interrupt.
    pub(self) const fn reserved() -> Self {
        Self { res: 0, }
    }
}
