//! I/O Configuration module.



use super::common::*;



pub(crate) struct IOControl;

// Metadata, constants, addresses and other necessary data.
impl IOControl {
    /// Base address of the IOCON peripheral.
    const IOCON: u32 = 0x40001000;
}

// Common register fields for all GPIO pins, regardless of type.
impl IOControl {
    /// Sets the alternate function of the given pin.
    #[inline]
    pub(super) fn setfn(port: u32, pin: u8, function: u32) {
        Self::modify(port, pin, function, 0xF)
    }

    /// Sets the pull resistors of the given pin.
    #[inline]
    pub(super) fn setpull(port: u32, pin: u8, mode: PullMode) {
        Self::modify(port, pin, (mode as u32) << 4, 0x3 << 4)
    }

    /// Sets the slew rate of the given pin.
    #[inline]
    pub(super) fn setspeed(port: u32, pin: u8, rate: SlewRate) {
        Self::modify(port, pin, (rate as u32) << 6, 1 << 6)
    }

    /// Sets the input polarity of the given pin.
    #[inline]
    pub(super) fn setpolarity(port: u32, pin: u8, polarity: Polarity) {
        Self::modify(port, pin, (polarity as u32) << 7 , 1 << 7)
    }

    /// Enable or disable the digital mode of the given pin.
    #[inline]
    pub(super) fn setdigital(port: u32, pin: u8, d: bool) {
        Self::modify(port, pin, if d { 1 << 8 } else { 0 }, 1 << 8)
    }

    /// Sets the pin mode of the given pin.
    #[inline]
    pub(super) fn setmode(port: u32, pin: u8, mode: GPIOMode) {
        Self::modify(port, pin, (mode as u32) << 9, 1 << 9)
    }

    /// Internal function to modify a register value.
    #[inline(never)]
    fn modify(port: u32, pin: u8, value: u32, mask: u32) {
        use core::ptr::{
            read_volatile as read,
            write_volatile as write,
        };

        // Calculate the offset.
        let offset = ((port * 32) + pin as u32) * 4;

        // Create the register pointer.
        let ptr = (Self::IOCON + offset) as *mut u32;

        unsafe {
            // Read the current value of the register.
            let mut v = read(ptr as *const u32);

            // Clear the mask and set the new value.
            v = (v & !mask) | (value & mask);

            // Write the modified value to the register.
            write(ptr, v);
        }
    }
}

impl crate::system::Control for IOControl {
    const REG: u32 = 0;
    const OFF: u8 = 13;
}

impl crate::system::enable::Enable for IOControl {}
impl crate::system::reset::Unreset for IOControl {}
