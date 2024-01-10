//! Core 0 exceptions.



/// A set of all fault reasons.
#[link_section = ".bss.HARDFAULT.reasons"]
#[used]
pub static mut REASONS: [u64; 6] = [0; 6];



/// Hard Fault exception handler.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Handler() {
    // Get all possible reasons of the HardFault.
    let bus = super::bus::BusFault::get();
    let mem = super::mem::MemFault::get();
    let usage = super::usage::UsageFault::get();
    let secure = super::secure::SecureFault::get();
    let hard = super::hardfault::HardFault::get();

    unsafe {
        let common = core::ptr::read_volatile(0xE000ED28 as *const u32);
        REASONS = [hard.state() as u64, bus.state() as u64, mem.state() as u64, usage.state() as u64, secure.state() as u64, common as u64];
    }

    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}

