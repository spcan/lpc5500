//! Clock control module.
//! NOTES :
//! 
//! ADC Clock max is 24 MHz
//! 
//! PLL1 reserved for main clock.



pub mod main;



/// Initializes the clocks of the device.
pub(crate) fn init() {
    use core::ptr::{
        read_volatile  as read,
        write_volatile as write,
    };

    #[cfg(feature = "defmt")]
    defmt::trace!("Powering up internal FROs...");

    unsafe {
        // Power up control in ANACTRL - FRO192M CONTROL
        // Enables the 96 MHz [30], 48 MHz [15] and 12 MHz [14] clocks.
        let mut fro192 = read(0x50013010 as *const u32);
        fro192 |= (1 << 30) | (1 << 15) | (1 << 14);
        write( 0x50013010 as *mut u32, fro192 );

        // Power up control in SYSCON - CLOCK CONTROL.
        // Enables the 12 Mhz FRO [7], the 1 Mhz FRO [6] nad the UTick clock [2].
        let mut clkctrl = read(0x50000A18 as *const u32);
        clkctrl |= (1 << 7) | (1 << 6) | (1 << 2);
        write( 0x50000A18 as *mut u32, clkctrl );

        // Disable Power Down in PMC - PDRUNCFG
        // Enables the 32 kHz FRO [6] and the 192 MHz FRO [5].
        write( 0x400200C8 as *mut u32, (1 << 6) | (1 << 5) );
    }

    #[cfg(feature = "defmt")]
    defmt::trace!("FROs are enabled");
}
