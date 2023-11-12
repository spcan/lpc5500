//! Internal FRO at 1 Mhz.



pub struct FRO1MHz;

impl FRO1MHz {
    /// SYSCON peripheral base.
    const SYSCON: usize = 0x50000000;

    /// CLOCK CTRL register offset.
    const CLOCKCTRL: usize = 0xA18;


    /// Initializes the 192 MHz FRO.
    pub(super) fn init() {
        use core::ptr::{
            read_volatile as read,
            write_volatile as write,
        };

        // Enable the 12 MHz [14], 48 MHz [15] and 96 MHz [30] clocks.
        unsafe {
            // Create the pointer.
            let ptr = (Self::SYSCON + Self::CLOCKCTRL) as *mut u32;

            // Read the current value.
            let v = read(ptr as *const u32);

            // Enable the FRO for the UTICK clock [bit 2] and for clock muxing [bit 6].
            write(ptr, v | (1 << 6) | (1 << 2));
        }
    }
}
