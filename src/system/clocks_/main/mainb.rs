//! Main clock selector B module.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



/// Current frequency of the main clock.
#[link_section = ".data.LPC5500.CLOCKS"]
pub(super) static FREQUENCY: AtomicU32 = AtomicU32::new( 0 );



/// Main clock selector B.
/// The source clock of this clock signal can be changed on the fly without
/// glitches to provide dynamic frequency scaling.
pub struct MainSelectB {
    /// The current clock signal source.
    source: Source,
}

impl MainSelectB {
    /// Static initializer.
    pub(super) const fn new() -> Self {
        Self { source: Source::MainSelectA, }
    }
}

impl super::super::ClockSignal for MainSelectB {
    type Token = ();

    fn freqin(&self) -> u32 {
        FREQUENCY.load( Ordering::Relaxed )
    }

    fn freqout(&self) -> u32 {
        self.freqin()
    }

    fn token(&mut self) -> Self::Token {
        ()
    }
}

impl MainSelectB {
    /// Sets the source of the clock (if the clock is not yet locked).
    /// Returns `true` if the operation was successful.
    pub fn source<S: SourceTrait>(&mut self, source: S) -> bool {
        // The address of the selection register.
        // SYSCON->MAINCLKSELB @ 0x50000000 + 0x284.
        const ADDRESS: usize = 0x50000000 + 0x284;

        // Get the source clock frequency.
        let freq = match source.value() {
            Source::MainSelectA => super::maina::FREQUENCY.load( Ordering::Relaxed ),
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



/// Common trait for all the clock signals that can be a source for the Main Selector B.
pub trait SourceTrait {
    /// Returns the `enum` value of this source.
    fn value(&self) -> Source;
}



/// List of all sources of the Main Selector B clock signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Source {
    /// Main Select A clock signal.
    MainSelectA = 0b00,

    /// Internal PLL 0.
    PLL0 = 0b01,

    /// Internal PLL 1.
    PLL1 = 0b10,

    /// Low power 32 kHz oscillator.
    Osc32k = 0b11,
}
