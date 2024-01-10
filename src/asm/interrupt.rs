//! Interrupt related assembly instructions.



#[inline(always)]
pub(crate) fn disable() -> InterruptGuard {
    // Get the PRIMASK before the interrupts were disabled.
    let primask = unsafe {
        // Read the PRIMASK register.
        let r: u32;
        core::arch::asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags));

        // Disable interrupts.
        core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));

        r
    };

    // Strict memory accesses.
    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    InterruptGuard( primask )
}



/// Interrupt guard with PRIMASK state.
pub struct InterruptGuard(pub(self) u32);

impl Drop for InterruptGuard {
    #[inline(always)]
    fn drop(&mut self) {
        // Check if PRIMASK was active.
        if (self.0 & 1) != 0 {
            // Enable interrupts.
            unsafe { core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags)) };

            // Strict memory accesses.
            core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );
        }
    }
}
