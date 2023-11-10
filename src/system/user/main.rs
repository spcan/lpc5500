//! Main clock configuration.



use core::sync::atomic::{
    AtomicU32, Ordering,
};


#[link_section = ".bss.LPC5500.user.MAINCLK"]
static FREQ: AtomicU32 = AtomicU32::new( 0 );


pub struct MainClock {
    /// Current clock source.
    source: ClockSource,
}

impl MainClock {
    /// Gets the current main clock frequency and source.
    pub(super) fn getclock(&self) -> (u32, ClockSource) {
        (Self::frequency(), self.source)
    }

    /// Returns the current main clock frequency.
    pub fn frequency() -> u32 {
        FREQ.load( Ordering::Relaxed )
    }
}



/// All clock sources allowed for the main clock.
#[derive(Clone, Copy, Eq, PartialEq)]
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
