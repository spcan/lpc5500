//! Vector implementation.



/// Union vector. Contains either reserved or raw values or function pointers.
#[derive(Clone, Copy)]
#[repr(C)]
pub union Vector {
    /// Existing interrupt.
    ptr: unsafe extern "C" fn(),

    /// Raw value interrupt.
    raw: u32,
}

impl Vector {
    /// Creates a vector from a function handler.
    pub const fn handler(ptr: unsafe extern "C" fn()) -> Self {
        Self { ptr }
    }

    /// Creates a vector from a raw value (does not perform type checking).
    pub const fn raw(raw: u32) -> Self {
        Self { raw }
    }

    /// Creates a reserved interrupt.
    pub const fn reserved() -> Self {
        Self { raw: 0 }
    }
}


#[cfg(feature = "defmt")]
impl defmt::Format for Vector {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "0x{:08X}", unsafe { self.raw } );
    }
}
