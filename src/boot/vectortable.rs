//! LPC5500 reset vector table.



#![allow(dead_code)]



/// Reset vector table.
#[link_section = ".exceptions"]
pub(super) static VTABLE: [Vector; 14] = [
    Vector::reserved(),
    Vector::reserved(),

    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),

    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),

    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
];



/// Vector abstraction.
pub(super) union Vector {
    /// A function pointer.
    f: fn(),

    /// A reserved entry.
    r: u32,
}

impl Vector {
    /// Creates a reserved Vector Table entry.
    pub(super) const fn reserved() -> Self {
        Vector { r: 0u32, }
    }

    /// Creates the Stack Pointer Vector Table entry.
    pub(super) const fn sp(r: u32) -> Self {
        Vector { r }
    }

    /// Creates a Vector Table entry.
    pub(super) const fn function(f: fn()) -> Self {
        Vector { f }
    }
}
