//! Random Number Generation module.



use crate::system::{
    Control,
    enable::Enable,
    reset::Unreset,
};


/// Provides a random number.
/// According to NXP it does not matter how often you call this method, as the
/// entropy will always be enough to pass classical security tests.
#[inline(always)]
pub fn random() -> u32 {
    unsafe { core::ptr::read_volatile( 0x5003A000 as *const u32 ) }
}



/// RNG subsystem token to initialize it.
/// Should be optimized away.
pub struct RNG;

impl Control for RNG {
    const REG: u32 = 2;
    const OFF: u8 = 13;
}

impl Enable  for RNG {}
impl Unreset for RNG {}
