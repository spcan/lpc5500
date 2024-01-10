//! General implementation of a vector table.



use super::vector::Vector;



#[repr(C)]
pub struct VectorTable<const N: usize> {
    /// Exceptions of the vector table.
    exceptions: [Vector; 16],

    /// Interrupts of the vector table.
    interrupts: [Vector; N],
}

impl<const N: usize> VectorTable<N> {
    /// Creates an empty `VectorTable`.
    /// This method is compatible with static initialization.
    pub const fn create() -> Self {
        VectorTable {
            exceptions: [Vector::reserved(); 16],
            interrupts: [Vector::reserved();  N],
        }
    }

    /// Copies the exception table of the given vector table.
    pub unsafe fn copyexc<const M: usize>(&mut self, other: &VectorTable<M>) {
        for i in 0..self.exceptions.len() {
            self.exceptions[i] = other.exceptions[i];
        }
    }

    /// Registers the given exception in the table.
    /// Optionally sets its priority.
    pub unsafe fn exception(&mut self, n: usize, exc: Vector) {
        self.exceptions[n] = exc;
    }

    /// Registers the given interrupt in the table.
    /// Enables the interrupt in the NVIC and optionally sets its priority.
    pub unsafe fn interrupt(&mut self, n: usize, int: Vector, priority: Option<u8>) {
        // Base address of the Interrupt Set Enable Register (ISER).
        const ISER: usize = 0xE000E100;

        // Register the interrupt.
        self.interrupts[n] = int;

        // Enable the interrupt.
        let a = n / 32;
        let b = n % 32;

        unsafe { core::ptr::write_volatile((ISER + (4 * a)) as *mut u32, 1 << b) }

        // Optionally set its priority.
        if let Some(p) = priority {
            // Base address of the Interrupt Priority Register (IPR).
            const IPR: usize = 0xE000E400;

            // Set the priority.
            let a = n / 4;
            let b = n % 4;

            unsafe { core::ptr::write_volatile( (IPR + (4 * a)) as *mut u32, (p as u32) << (4 * b)) }
        }
    }
}

impl VectorTable<0> {
    /// Creates an initialized exceptions-only `VectorTable`.
    pub const fn preloaded(exceptions: [Vector; 16]) -> Self {
        VectorTable {
            exceptions,
            interrupts: [Vector::reserved(); 0],
        }
    }
}


#[cfg(feature = "defmt")]
impl<const N: usize> defmt::Format for VectorTable<N> {
    fn format(&self, f: defmt::Formatter) {
        // Header with Vector Table address.
        fmt(f, &self.exceptions, &self.interrupts[..]);
    }
}



/// Common log function for all Vtable implementations.
#[cfg(feature = "defmt")]
#[inline(never)]
fn fmt(fmt: defmt::Formatter, exc: &[Vector; 16], int: &[Vector]) {
    use defmt::write;

    // Header with Vector Table address.
    write!(fmt, "Vector Table @ 0x{:X}\nExceptions:\n", exc as *const _ as u32);

    // Log the exceptions.
    write!(
        fmt,
        "  SP     : {}\n  Reset  : {}\n  NMI    : {}\n  Hard   : {}\n  Mem    : {}\n  Bus    : {}\n  Usage  : {}\n  Secure : {}\n  SVC    : {}\n  Debug  : {}\n  PendSV : {}\n  Systick: {}\n",
        exc[0], exc[1], exc[2], exc[3], exc[4], exc[5], exc[6], exc[7], exc[11], exc[12], exc[14], exc[15],
    );

    // Log the interrupts.
    write!(fmt, "Interrupts:");

    for (i, vector) in int.iter().enumerate() {
        write!(fmt, "\n  {:03}: {}", i, vector);
    }
}
