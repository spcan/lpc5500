//! A reader of the OS Timer value.



#[derive(Clone, Copy)]
pub struct OSTime;

impl OSTime {
    /// Unsafe static initializer.
    pub(crate) const unsafe fn uninit() -> Self {
        Self
    }

    /// Reads the current value of the OS timer (with 1-3 cycle latency).
    /// This method is always safe after initialization.
    #[inline(never)]
    pub fn read(&self) -> u64 {
        unsafe {
            // Disable interrupts and use SEV to capture the value.
            //core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));

            // Enforce a compiler fence.
            core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

            // Get the value.
            let lo = u64::from( core::ptr::read_volatile(0x4002D000 as *const u32) );
            let hi = u64::from( core::ptr::read_volatile(0x4002D004 as *const u32) );

            // Enforce a compiler fence.
            core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

            // Reenable interrupts.
            //core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));

            // Calculate the decimal timer value.
            decode( lo | (hi << 32) )
        }
    }

    fn decimal(mut gray: u64) -> u64 {
        // Copy the gray value.
        let mut temp = gray;

        while temp > 0 {
            // Shift right temp.
            temp >>= 1;

            // XOR the gray value.
            gray ^= temp;
        }

        gray
    }
}


fn decode(gray: u64) -> u64 {
    match gray {
        0 => 0,
        _ => gray ^ decode(gray >> 1)
    }
}

fn decimal_(mut gray: u64) -> u64 {
    // Copy the gray value.
    let mut temp = gray;

    while temp > 0 {
        // Shift right temp.
        temp >>= 1;

        // XOR the gray value.
        gray ^= temp;
    }

    gray
}
