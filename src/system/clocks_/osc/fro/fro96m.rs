//! 96 MHz Free Running Oscillator module.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



/// Current frequency of the main clock.
#[link_section = ".data.LPC5500.CLOCKS"]
pub(super) static FREQUENCY: AtomicU32 = AtomicU32::new( 0 );



/// Internal FRO running at 96 MHz.
pub struct FRO96MHz;

impl crate::system::clocks::ClockEnable for FRO96MHz {
    fn enable(&mut self) {
        // Enable the clock in the FRO.
        super::FROControl::enable::<Self>();

        // Set the frequency.
        FREQUENCY.store(96_000_000, Ordering::Relaxed);
    }
}

impl crate::system::clocks::ClockSignal for FRO96MHz {
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

impl super::FROEnable for FRO96MHz {
    const OFFSET: u8 = 14;
}

impl super::FRODisable for FRO96MHz {}



/// A token representing the output of the 1 MHz FRO clock signal.
pub struct Token;

impl crate::system::clocks::main::maina::SourceTrait for Token {
    fn value(&self) -> crate::system::clocks::main::maina::Source {
        crate::system::clocks::main::maina::Source::FRO96MHz
    }
}
