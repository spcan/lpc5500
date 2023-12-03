//! Usage Fault exception handler.



#![allow(non_snake_case)]



/// Function handler of the Usage Fault exception.
pub(super) fn Handler() {
    // Get the Usage Fault information.
    let info = UsageFault::get();

    // TODO : Include here user code.
}



/// Contains all information of a Bus Fault.
/// <div class="warning">WARNING : When this struct is dropped it clears the Bus Fault flags. To
/// avoid clearing the flags use `BusFault::forget()`</div>
pub struct UsageFault(u32);

impl UsageFault {
    /// Reads the information of the Usage Fault.
    pub(self) fn get() -> Self {
        // Get the UFSR.
        let state = (super::cfsr() >> 8) & 0xFF;

        Self(state)
    }

    /// Returns `true` if the given Usage Fault reason appears in the Usage Fault status.
    pub fn contains(&self, reason: UsageFaultReason) -> bool {
        (self.0 & (1 << reason as u8)) != 0
    }

    /// Drops the Bus Fault without clearing the sticky flags.
    pub fn forget(self) {
        core::mem::forget(self)
    }
}

impl Drop for Usage {
    fn drop(mut self) {
        // Clear the sticky flags.
        unsafe { core::ptr::write_volatile( 0xE000ED28 as *mut u32, self.0 << 16 ) }
    }
}



/// List of all possible reasons for a triggered Usage Fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsageFaultReason {
    /// Undefined Instruction.
    /// An undefined instruction was executed.
    UndefinedInstruction = 0,

    /// Invalid State.
    /// EPSR.T or EPSR.IT was in an invalid state.
    InvalidState = 1,

    /// Invalid PC.
    /// An integrity check on the Program Counter failed.
    InvalidPC = 2,

    /// No Coprocessor.
    /// The coprocessor accessed either is disabled or does not exist.
    NoCoprocessor = 3,

    /// Stack Overflow.
    StackOverflow = 4,

    /// Unaligned Access.
    UnalignedAccess = 8,

    /// Divide by Zero.
    DivideByZero = 9,
}
