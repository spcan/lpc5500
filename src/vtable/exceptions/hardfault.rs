//! HardFault exception.


#[link_section = ".bss.HARDFAULT.reasons"]
#[used]
pub static mut REASONS: [u64; 6] = [0; 6];


/// Function handler of the Hardfault exception.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Handler() {
    // Get all possible reasons of the HardFault.
    let bus = super::bus::BusFault::get();
    let mem = super::mem::MemFault::get();
    let usage = super::usage::UsageFault::get();
    let secure = super::secure::SecureFault::get();
    let hard = HardFault::get();

    unsafe {
        let common = core::ptr::read_volatile(0xE000ED28 as *const u32);
        REASONS = [hard.state() as u64, bus.state() as u64, mem.state() as u64, usage.state() as u64, secure.state() as u64, common as u64];
    }

    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}



/// Contains all information of a hard Fault.
/// <div class="warning">WARNING : When this struct is dropped it clears the Hard Fault flags. To
/// avoid clearing the flags use `HardFault::forget()`</div>
pub struct HardFault(u32);

impl HardFault {
    /// Reads the information of the Usage Fault.
    pub(super) fn get() -> Self {
        // Get the UFSR.
        let state = super::hfsr();

        Self(state)
    }

    /// Returns the state of the Hard Fault (if valid).
    pub fn state(&self) -> u32 {
        self.0
    }

    /// Returns `true` if the given Usage Fault reason appears in the Usage Fault status.
    pub fn contains(&self, reason: HardFaultReason) -> bool {
        (self.0 & (1 << reason as u8)) != 0
    }

    /// Drops the Bus Fault without clearing the sticky flags.
    pub fn forget(self) {
        core::mem::forget(self)
    }
}

impl Drop for HardFault {
    fn drop(&mut self) {
        // Clear the sticky flags.
        unsafe { core::ptr::write_volatile( 0xE000ED2C as *mut u32, self.0 << 16 ) }
    }
}



/// List of all possible reasons for a triggered Hard Fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardFaultReason {
    /// Vector Table read error on exception processing.
    VectorTable = 1,

    /// A fault with configurable priority has been escalated to a HardFault
    /// because it could not be made active (because of priority or it was disabled).
    Forced = 30,

    /// Indicates a debug event has ocurred.
    Debug = 31,
}
