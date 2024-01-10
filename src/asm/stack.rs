//! Stack Pointer related assembly instructions.



pub(crate) struct MSPLimit;

impl MSPLimit {
    /// Sets the MSP Limit.
    #[inline(always)]
    pub unsafe fn set(limit: u32) {
        core::arch::asm!("msr MSPLIM, {}", in(reg) limit, options(nomem, nostack, preserves_flags))
    }
}


pub(crate) struct PSPLimit;

impl PSPLimit {
    /// Sets the PSP Limit.
    #[inline(always)]
    pub unsafe fn set(limit: u32) {
        core::arch::asm!("msr PSPLIM, {}", in(reg) limit, options(nomem, nostack, preserves_flags))
    }
}
