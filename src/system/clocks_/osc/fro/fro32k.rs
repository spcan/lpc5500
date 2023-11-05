//! 32 kHz Free Running Oscillator module.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



/// Current frequency of the main clock.
#[link_section = ".data.LPC5500.CLOCKS"]
pub(super) static FREQUENCY: AtomicU32 = AtomicU32::new( 0 );



pub struct FRO32KHz;

impl crate::system::clocks::ClockEnable for FRO32KHz {
    fn enable(&mut self) {
        // Power on the clock.
        self.poweron();

        // Set the frequency.
        FREQUENCY.store(1_000_000, Ordering::Relaxed);
    }
}

impl crate::system::clocks::ClockSignal for FRO32KHz {
    type Token = Token;

    fn freqin(&self) -> u32 {
        0
    }

    fn freqout(&self) -> u32 {
        FREQUENCY.load( Ordering::Relaxed )
    }

    fn token(&mut self) -> Self::Token {
        Token
    }
}



/// A token representing the output of the 1 MHz FRO clock signal.
pub struct Token;
