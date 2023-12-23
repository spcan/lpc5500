//! Secure exception.



/// Function handler of the Secure exception.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Handler() {
    // Read the secure fault.
    //let fault = SecureFault::get();

    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}


/// Contains all information of a Secure Management Fault.
/// <div class="warning">WARNING : When this struct is dropped it clears the Bus Fault flags. To
/// avoid clearing the flags use `SecureFault::forget()`</div>
pub struct SecureFault {
    /// The Mem Fault State Register.
    state: u32,

    /// The address that caused the Mem Fault.
    address: u32,
}

impl SecureFault {
    /// Reads the information of the Mem Fault.
    pub(super) fn get() -> Self {
        // Get the SFSR.
        let state = super::sfsr() & 0x7F;

        // Read the Secure Fault address.
        let address = unsafe { core::ptr::read_volatile( 0xE000EDE8 as *const u32 ) };

        Self { state, address, }
    }

    /// Returns the specific address of the Secure Fault (if valid).
    pub fn address(&self) -> Option<u32> {
        // Check if the BFAR Valid bit is set.
        if (self.state & (1 << 6)) == 0 { return None }

        // Read the BFAR.
        Some( self.address )
    }

    /// Returns the state of the Fault (if valid).
    pub fn state(&self) -> u32 {
        self.state
    }

    /// Returns `true` if the given Mem Fault reason appears in the Bus Fault status.
    pub fn contains(&self, reason: SecureFaultReason) -> bool {
        (self.state & (1 << reason as u8)) != 0
    }

    /// Drops the Mem Fault without clearing the sticky flags.
    pub fn forget(self) {
        core::mem::forget(self)
    }
}

impl Drop for SecureFault {
    fn drop(&mut self) {
        // Clear the sticky flags.
        unsafe { core::ptr::write_volatile( 0xE000EDE8 as *mut u32, self.state ) }
    }
}



/// List of all possible reasons for a triggered Secure Fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureFaultReason {
    /// A function call from the non-secure state or exception targets a non-SG instruction
    /// in the secure state.
    InvalidEntry = 0,

    /// Invalid integrity signature in an exception stack frame.
    InvalidSignature = 1,

    /// Invalid exception return flag. Caused by EXCRETURN->DCRS = 0 or EXCRETURN->ES = 1 when
    /// returning from an exception in non-secure state. 
    InvalidExcReturn = 2,

    /// Indicates that an attempt was made to access parts of the address space marked as Secure.
    AttributionViolation = 3,

    /// A branch was not flagged as domain crossing when it transitions from Secure to Non-Secure.
    InvalidTransition = 4,

    /// Lazy state preservation for Floating Point registers caused an access violation.
    FPLazy = 5,

    /// Sticky flag indicating that an error ocurred during lazy state activation or deactivation.
    LazyState = 7,
}
