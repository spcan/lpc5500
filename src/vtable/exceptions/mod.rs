



pub(super) mod bus;
pub(super) mod debug;
pub(super) mod hardfault;
pub(super) mod mem;
pub(super) mod nmi;
pub(super) mod pendsv;
pub(super) mod secure;
pub(super) mod svcall;
pub(super) mod systick;
pub(super) mod usage;



use super::Vector;



/// Reads the Configurable Fault Status Register.
#[inline(always)]
fn cfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED28 as *const u32) }
}

/// Reads the Secure Fault Status Register.
#[inline(always)]
fn sfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000EDE4 as *const u32) }
}

/// Reads the Hard Fault Status Register.
#[inline(always)]
fn hfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED2C as *const u32) }
}



/// Default handler to block execution.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Block() {
    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}



/// Default handler to ignore exception.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Ignore() {
    return
}
