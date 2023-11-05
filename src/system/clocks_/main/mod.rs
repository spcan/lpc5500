//! Main clock configuration.



mod control;
pub(crate) mod maina;
pub(crate) mod mainb;



pub use control::ClockControl;

use core::sync::atomic::{
    AtomicU32, Ordering,
};



/// Current frequency of the main clock.
#[link_section = ".data.LPC5500.CLOCKS"]
static FREQUENCY: AtomicU32 = AtomicU32::new( 0 );



/// Main clock.
/// Provides the clock signal to the CPU cores, the AHB bus, the memories...
/// The source clock of this clock signal (MainSelectB) can be changed on the
/// fly without glitches to provide dynamic frequency scaling.
pub struct Main;

impl Main {
    /// Static initializer.
    pub const fn new() -> Self {
        Self
    }
}

impl super::ClockSignal for Main {
    type Token = ();

    fn freqin(&self) -> u32 {
        mainb::FREQUENCY.load( Ordering::Relaxed )
    }

    fn freqout(&self) -> u32 {
        FREQUENCY.load( Ordering::Relaxed )
    }

    fn token(&mut self) -> Self::Token {
        ()
    }
}

impl super::DivClock for Main {
    type Divider = u8;

    /// Divides the Main Clock Select B clock signal by the given value plus 1.
    /// For no division set the value 0 (DIV = 0 + 1 = 1).
    fn divide(&mut self, div: u8) {
        use super::ClockSignal;

        // Address of the register.
        // SYSCON->AHBCLKDIV @ 0x50000000 + 0x380
        const ADDRESS: usize = 0x50000000 + 0x380;

        unsafe {
            // Write the new divider.
            core::ptr::write_volatile(ADDRESS as *mut u32, div as u32)
        }

        // Set the new frequency.
        FREQUENCY.store(self.freqin() / (div as u32), Ordering::Relaxed);
    }
}
