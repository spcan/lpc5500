//! Bus Fault exception handler.



#![allow(non_snake_case)]



/// Function handler of the Bus Fault exception.
pub(super) fn Handler() {
    // Get the Bus Fault information.
    let info = BusFault::get();

    // TODO : Include here user code.
}



/// Contains all information of a Bus Fault.
/// <div class="warning">WARNING : When this struct is dropped it clears the Bus Fault flags. To
/// avoid clearing the flags use `BusFault::forget()`</div>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BusFault {
    /// The Bus Fault State Register.
    state: u32,

    /// The address that caused the Bus Fault.
    address: u32,
}

impl BusFault {
    /// Reads the information of the Bus Fault.
    pub(self) fn get() -> Self {
        // Get the BFSR.
        let state = (super::cfsr() >> 8) & 0xFF;

        // Read the Bus Fault address.
        let address = unsafe { core::ptr::read_volatile( 0xE000ED38 as *const u32 ) };

        Self { state, address, }
    }

    /// Returns the specific address of the Bus Fault (if valid).
    pub fn address(&self) -> Option<u32> {
        // Check if the BFAR Valid bit is set.
        if (self.state & (1 << 7)) == 0 { return None }

        // Read the BFAR.
        Some( self.address )
    }

    /// Returns `true` if the given Bus Fault reason appears in the Bus Fault status.
    pub fn contains(&self, reason: BusFaultReason) -> bool {
        (self.state & (1 << reason as u8)) != 0
    }

    /// Drops the Bus Fault without clearing the sticky flags.
    pub fn forget(self) {
        core::mem::forget(self)
    }
}

impl Drop for BusFault {
    fn drop(mut self) {
        // Clear the sticky flags.
        unsafe { core::ptr::write_volatile( 0xE000ED28 as *mut u32, self.state << 8 ) }
    }
}



/// List of all possible reasons for a triggered Bus Fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BusFaultReason {
    /// The processor detects the instruction bus error on prefetching an instruction.
    Instruction = 0,

    /// A data bus error with a precise address.
    /// The return address points to the instruction that caused the fault.
    Precise = 1,

    /// Unstacking from an exception return caused one or more Bus Faults.
    /// Returning from this exception does not return to the previous exception
    /// but to the context before the faulting exception.
    Unstacking = 3,

    /// Stacking for an exception entry caused  one or more Bus Faults.
    /// The Stack Pointer register is adjusted but the values in the target
    /// context area might be incorrect.
    Stacking = 4,

    /// Lazy state preservation for Floating Point registers caused a Bus Fault.
    FPLazy = 5,
}
