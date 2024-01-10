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
