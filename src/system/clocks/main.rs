//! Main clock control.
//! This interface is available to the user to implement dynamic frequency scaling.



use core::{
    ptr::{
        read_volatile as read,
        write_volatile as write,
    },

    sync::atomic::{
        AtomicU32, Ordering,
    },
};



#[link_section = ".bss.LPC5500.user.MAINCLK"]
pub(crate) static FREQ: AtomicU32 = AtomicU32::new( 0 );



/// User facing clock interface.
pub struct MainClock {
    /// Current frequency source.
    source: ClockSource,

    // PLL1 control.
    //pll1: ,
}

impl MainClock {
    /// SYSCON peripheral base address.
    const SYSCON: u32 = 0x50000000;

    /// Main Clock selector A register.
    const MAINSELA: u32 = Self::SYSCON + 0x280;

    /// Main Clock selector B register.
    const MAINSELB: u32 = Self::SYSCON + 0x284;

    /// AHB bus clock divisor.
    const AHBDIV: u32 = Self::SYSCON + 0x380;

    /// Initializes the main clock control.
    pub(crate) fn init() -> Self {
        // Read the current source.
        let source = match unsafe { read(Self::MAINSELB as *const u32) } {
            0 => match unsafe { read(Self::MAINSELA as *const u32) } {
                0 => ClockSource::FRO12Mhz,

                1 => ClockSource::FRO1Mhz,

                2 => ClockSource::FRO1Mhz,

                _ => ClockSource::FRO96Mhz,
            },

            1 => ClockSource::FRO1Mhz,

            2 => ClockSource::FRO1Mhz,

            _ => ClockSource::FRO1Mhz,
        };

        // Set the current frequency.
        FREQ.store(source.frequency(), Ordering::SeqCst);

        // Set AHBCLOCKDIV to null.
        unsafe { core::ptr::write_volatile(Self::AHBDIV as *mut u32, 1) }

        Self { source, }
    }

    /// Gets the current main clock frequency and source.
    pub fn getclock(&self) -> (u32, ClockSource) {
        (Self::frequency(), self.source)
    }

    /// Returns the current frequency of the Main Clock signal.
    pub fn frequency() -> u32 {
        FREQ.load( Ordering::Relaxed )
    }

    /// Switches the main clock to the given source.
    pub(crate) fn switch(&mut self, source: ClockSource) {
        // If the clock is the PLL 1 ensure it is enabled and running.

        // Perform the switching.
        match source {
            ClockSource::FRO12Mhz => unsafe {
                write( Self::MAINSELA as *mut u32, 0 );
                write( Self::MAINSELB as *mut u32, 0 );
            },

            ClockSource::FRO1Mhz => unsafe {
                write( Self::MAINSELA as *mut u32, 2 );
                write( Self::MAINSELB as *mut u32, 0 );
            },

            ClockSource::FRO96Mhz => unsafe {
                write( Self::MAINSELA as *mut u32, 3 );
                write( Self::MAINSELB as *mut u32, 0 );
            },

            /*
            ClockSource::PLL1 => unsafe {
                write( Self::MAINSELB as *mut u32, 2 );
            },

            ClockSource::FRO32kHz => unsafe {
                write( Self::MAINSELB as *mut u32, 3 );
            },
            */
        }

        // Store the new source.
        self.source = source;

        // Update the frequency.
        FREQ.store(source.frequency(), Ordering::SeqCst);
    }
}



/// All clock sources allowed for the main clock.
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// Internal FRO at 1 MHz.
    FRO1Mhz,

    /// Internal FRO at 12 MHz.
    FRO12Mhz,

    /// Internal FRO at 96 MHz.
    FRO96Mhz,
}

impl ClockSource {
    /// Returns the current frequency of the clock source.
    pub fn frequency(&self) -> u32 {
        match self {
            ClockSource::FRO1Mhz  =>  1_000_000,
            ClockSource::FRO12Mhz => 12_000_000,
            ClockSource::FRO96Mhz => 96_000_000,
        }
    }
}
