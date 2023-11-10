//! Internal FRO at 32 kHz.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



#[link_section = ".bss.LPC5500.clocks.FRO"]
static FREQUENCY : AtomicU32 = AtomicU32::new(0);

#[link_section = ".bss.LPC5500.clocks.FRO"]
static REFCNT: AtomicU32 = AtomicU32::new(0);




pub struct FRO32kHz;

impl FRO32kHz {
    /// Initializes the 32 kHz FRO.
    pub(super) fn init() {
        
    }
}

impl crate::system::clocks::ClockTrait for FRO32kHz {
    type Token = Token;

    fn frequency(&self) -> u32 {
        FREQUENCY.load( Ordering::Relaxed )
    }

    fn token(&mut self) -> Self::Token {
        Token
    }
}


/// A token representing the 32 kHz FRO.
pub struct Token;
