//! Memory Management Fault exception handler.



#![allow(non_snake_case)]



/// Function handler of the MemManage Fault exception.
pub(super) unsafe extern "C" fn Handler() {
    // Get the Mem Fault information.
    let info = MemFault::get();

    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}



/// Contains all information of a Memory Management Fault.
/// <div class="warning">WARNING : When this struct is dropped it clears the Bus Fault flags. To
/// avoid clearing the flags use `MemFault::forget()`</div>
pub struct MemFault {
    /// The Mem Fault State Register.
    state: u32,

    /// The address that caused the Mem Fault.
    address: u32,
}

impl MemFault {
    /// Reads the information of the Mem Fault.
    pub(super) fn get() -> Self {
        // Get the MFSR.
        let state = super::cfsr() & 0xFF;

        // Read the Mem Fault address.
        let address = unsafe { core::ptr::read_volatile( 0xE000ED34 as *const u32 ) };

        Self { state, address, }
    }

    /// Returns the specific address of the Mem Fault (if valid).
    pub fn address(&self) -> Option<u32> {
        // Check if the BFAR Valid bit is set.
        if (self.state & (1 << 7)) == 0 { return None }

        // Read the BFAR.
        Some( self.address )
    }

    /// Returns the state of the Fault (if valid).
    pub fn state(&self) -> u32 {
        self.state
    }

    /// Returns `true` if the given Mem Fault reason appears in the Bus Fault status.
    pub fn contains(&self, reason: MemFaultReason) -> bool {
        (self.state & (1 << reason as u8)) != 0
    }

    /// Drops the Mem Fault without clearing the sticky flags.
    pub fn forget(self) {
        core::mem::forget(self)
    }
}

impl Drop for MemFault {
    fn drop(&mut self) {
        // Clear the sticky flags.
        unsafe { core::ptr::write_volatile( 0xE000ED28 as *mut u32, self.state ) }
    }
}



/// List of all possible reasons for a triggered Bus Fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemFaultReason {
    /// Attempted to execute instructions from a No Execution (NX) region.
    /// The stacked Program Counter register points to the faulting isntruction.
    InstructionAccess = 0,

    /// Attempted to load or store at a location that does not permite the operation.
    /// The stacked Program Counter register points to the faulting isntruction.
    DataAccess = 1,

    /// Unstacking from an exception return caused one or more access violations.
    /// Returning from this exception does not return to the previous exception
    /// but to the context before the faulting exception.
    Unstacking = 3,

    /// Stacking for an exception entry caused  one or more access violations.
    /// The Stack Pointer register is adjusted but the values in the target
    /// context area might be incorrect.
    Stacking = 4,

    /// Lazy state preservation for Floating Point registers caused an access violation.
    FPLazy = 5,
}
