//! GPIO Control peripheral.



pub struct GPIOControl;

// Metadata, constants, addresses and other necessary data.
impl GPIOControl {
    /// Base address of the GPIO peripheral.
    const GPIO: u32 = 0x4008C000;

    /// Offset of the DIRSET register block.
    const DIRSET: u32 = 0x2380;

    /// Offset of the DIRCLR register block.
    const DIRCLR: u32 = 0x2400;

    /// Offset of the SET register block.
    const SET: u32 = 0x2200;

    /// Offset of the CLR register block.
    const CLR: u32 = 0x2280;

    /// Offset of the NOT register block.
    const NOT: u32 = 0x2300;
}

impl GPIOControl {
    /// Sets the output of the pin to low level.
    pub(super) fn set(port: u32, pin: u8) {
        // The toggle register.
        let dst = (Self::GPIO + Self::SET) + (port * 4);

        unsafe {
            // Write to the set register.
            core::ptr::write_volatile(dst as *mut u32, 1 << pin);
        }
    }

    /// Sets the output of the pin to low level.
    pub(super) fn clear(port: u32, pin: u8) {
        // The toggle register.
        let dst = (Self::GPIO + Self::CLR) + (port * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << pin);
        }
    }

    /// Toggles the output of the pin.
    pub(super) fn toggle(port: u32, pin: u8) {
        // The toggle register.
        let dst = (Self::GPIO + Self::NOT) + (port * 4);

        unsafe {
            // Write to the toggle register.
            core::ptr::write_volatile(dst as *mut u32, 1 << pin);
        }
    }

    /// Sets the GPIO Pin as input.
    pub(super) fn input(port: u32, pin: u8) {
        // The direction clear register.
        let dst = (Self::GPIO + Self::DIRCLR) + (port * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << pin);
        }
    }

    /// Sets the GPIO Pin as output.
    pub(super) fn output(port: u32, pin: u8) {
        // The direction clear register.
        let dst = (Self::GPIO + Self::DIRSET) + (port * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << pin);
        }
    }
}
