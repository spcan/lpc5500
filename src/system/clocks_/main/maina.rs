//! Main clock selector A module.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



/// Current frequency of the main clock.
#[link_section = ".data.LPC5500.CLOCKS"]
pub(super) static FREQUENCY: AtomicU32 = AtomicU32::new( 0 );



/// Main clock selector A.
pub struct MainSelectA {
    /// The current clock signal source.
    source: Source,
}

impl MainSelectA {
    /// Static initializer.
    pub(super) const fn new() -> Self {
        Self { source: Source::FRO12MHz, }
    }
}

impl super::super::ClockSignal for MainSelectA {
    type Token = Token;

    fn freqin(&self) -> u32 {
        FREQUENCY.load( Ordering::Relaxed )
    }

    fn freqout(&self) -> u32 {
        self.freqin()
    }

    fn token(&mut self) -> Self::Token {
        Token
    }
}

impl MainSelectA {
    /// Sets the source of the clock (if the clock is not yet locked).
    /// Returns `true` if the operation was successful.
    pub fn source<S: SourceTrait>(&mut self, source: S) -> bool {
        // The address of the selection register.
        // SYSCON->MAINCLKSELA @ 0x50000000 + 0x280.
        const ADDRESS: usize = 0x50000000 + 0x280;

        // Get the source clock frequency.
        let freq = match source.value() {
            //Source::MainSelectA => super::maina::MAINAFREQ.load( Ordering::Relaxed ),
            //Source::PLL0 => crate::system::clocks::pll::PLL0FREQ.load( Ordering::Relaxed ),
            //Source::PLL1 => crate::system::clocks::pll::PLL1FREQ.load( Ordering::Relaxed ),
            _ => 32_000,
        };

        // Ensure the clock frequency is not 0.
        if freq == 0 { return false; }

        // Change the clock source.
        unsafe { core::ptr::write_volatile(ADDRESS as *mut u32, source.value() as u8 as u32) }

        // Update the clock signal frequency.
        FREQUENCY.store( freq, Ordering::Relaxed );

        true
    }
}



/// A token representing the output of the Main Select A clock signal.
pub struct Token;

impl super::mainb::SourceTrait for Token {
    fn value(&self) -> super::mainb::Source {
        super::mainb::Source::MainSelectA
    }
}



/// Common trait for all the clock signals that can be a source for the Main Selector A.
pub(crate) trait SourceTrait {
    /// Returns the `enum` value of this source.
    fn value(&self) -> Source;
}



/// List of all sources of the Main Selector B clock signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Source {
    /// FRO at 12 MHz.
    FRO12MHz = 0b00,

    /// Clock input.
    ClockIn = 0b01,

    /// FRO at 1 Mhz.
    FRO1MHz = 0b10,

    /// FRO at 96 MHz.
    FRO96MHz = 0b11,
}
