//! Internal FRO at 192 Mhz.



use crate::system::pdrun::PowerOn;



pub struct FRO192MHz;

impl FRO192MHz {
    /// ANACTRL peripheral base.
    const ANACTRL: usize = 0x50013000;

    /// FRO192MCTRL register offset.
    const FRO192MCTRL: usize = 0x10;


    /// Initializes the 192 MHz FRO.
    pub(super) fn init() {
        use core::ptr::{
            read_volatile as read,
            write_volatile as write,
        };

        // Power on the FRO.
        Self.poweron();

        // Enable the 12 MHz [14], 48 MHz [15] and 96 MHz [30] clocks.
        unsafe {
            // Create the pointer.
            let ptr = (Self::ANACTRL + Self::FRO192MCTRL) as *mut u32;

            // Read the current value.
            let v = read(ptr as *const u32);

            // Enable all clocks and store the modified register.
            write(ptr, v | (1 << 30) | (1 << 15) | (1 << 14));
        }
    }
}

impl PowerOn for FRO192MHz {
    const OFFSET: u8 = 5;
}
