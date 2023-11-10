//! Free Running Oscillator module.



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};


/// Internal control of the FRO clocks.
pub(super) struct FROControl;

impl FROControl {
    /// ANACTRL peripheral base.
    const ANACTRL: usize = 0x50013000;

    /// FRO192MCTRL register offset.
    const FRO192MCTRL: usize = 0x10;

    /// PMC peripheral base.
    const PMC: usize = 0x40020000;

    /// PDRUNCLR register offset.
    const PDRUNCLR: usize = 0xC8;

    /// Initializes the internal oscillators.
    pub (super) fn init() {
        // Start the 192 MHz FRO [PDRUN bit 5] and 32 kHz FRO [PDRUN bit 6].
        unsafe {
            // Create the pointer.
            let dst = (Self::PMC + Self::PDRUNCLR) as *mut u32;

            // Write to clear the bits.
            write(dst, (1 << 5) | (1 << 6));
        }

        // Enable the 12 MHz [14], 48 MHz [15] and 96 MHz [30] clocks.
        unsafe {
            // Create the pointer.
            let ptr = (Self::ANACTRL + Self::FRO192MCTRL) as *mut u32;

            // Read the current value.
            let v = read(ptr as *const u32);

            // Enable all clocks and store the modified register.
            write(ptr, v | (1 << 14) | (1 << 15) | (1 << 30));
        }
    }
}
