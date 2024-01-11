//! PowerQuad AHB engine interface.



pub(crate) mod handler;

pub mod matrix;

mod error;



pub use error::Error;

use matrix::MatrixOperation;

use self::matrix::MatrixTrait;



/// PowerQuad Coprocessor interface.
pub struct Engine;

impl Engine {
    /// Base address of the PowerQuad.
    const BASE: u32 = 0x500A6000;

    /// Adds two matrices, storing the result in the third matrix.
    /// BUG : CUrrently this method will cause a HardFault to trigger.
    pub fn add<A: MatrixTrait, B: MatrixTrait, C: MatrixTrait>(&mut self, a: &A, b: &B, c: &mut C) -> Result<(), MatrixError> {
        // Calculate the LENGTH register.
        let l = Self::mtxcheck3(a.dims(), b.dims(), c.dims(), MatrixCheck3::Equal)?;

        // Launch the operation.
        Self::mtxop(a.address(), b.address(), c.address(), l as u32, MatrixOperation::Add);

        Ok(())
    }

    /// Blocks the bus until the current operation finishes.
    pub fn finish(&mut self) {
        use core::ptr::read_volatile as read;

        // CONTROL register.
        const CONTROL: u32 = 0x100;

        while (unsafe { read((Self::BASE + CONTROL) as *const u32) } & (1 << 31)) != 0 {
            unsafe { core::arch::asm!("nop", options(nostack, nomem)) }
        }
    }

    /// Internal function to check a matrix operation with three operands.
    #[inline(never)]
    fn mtxcheck3(a: (usize, usize), b: (usize, usize), c: (usize, usize), op: MatrixCheck3) -> Result<u32, MatrixError> {
        match op {
            MatrixCheck3::Equal => {
                // Get the dimensions of A and B.
                let (acols, arows) = a;
                let (bcols, brows) = b;
                let (ccols, crows) = c;

                if ((acols ^ bcols ^ ccols) == acols) && ((arows ^ brows ^ crows) == arows) {
                    // Calculate the LENGTH register.
                    return Ok( Self::mtxlen(arows, acols, bcols) as u32 );
                }
            },

            MatrixCheck3::Multiply => {
                // Get the dimensions of A, B and C.
                let (acols, arows) = a;
                let (bcols, brows) = b;
                let (ccols, crows) = c;

                // Validate matrix dimensions.
                if (acols == brows) && (arows == crows) && (bcols == ccols) {
                    // Calculate the LENGTH register.
                    return Ok( Self::mtxlen(arows, acols, bcols) as u32 );
                }
            },
        }

        Err( MatrixError::IncompatibleDimensions )
    }

    /// Creates the LENGTH register for a powerquad operation.
    #[inline(always)]
    fn mtxlen(arows: usize, acols: usize, bcols: usize) -> usize {
        (bcols << 16) | (acols << 8) | (arows << 0)
    }

    /// Launches a given matrix operation with the given data.
    #[inline(never)]
    fn mtxop(a: u32, b: u32, c: u32, l: u32, o: MatrixOperation) {
        // OUTBASE register.
        const OUTBASE: u32 = 0x000;

        // TMPBASE register.
        const TMPBASE: u32 = 0x008;

        // INABASE register.
        const INABASE: u32 = 0x010;

        // INBBASE register.
        const INBBASE: u32 = 0x018;

        // CONTROL register.
        const CONTROL: u32 = 0x100;

        // LENGTH register.
        const LENGTH: u32 = 0x104;

        unsafe {
            // Write the Output matrix address.
            core::ptr::write_volatile((Self::BASE + OUTBASE) as *mut u32, c);

            // Write the Input A matrix address.
            core::ptr::write_volatile((Self::BASE + INABASE) as *mut u32, a);

            // Write the Input B matrix address.
            core::ptr::write_volatile((Self::BASE + INBBASE) as *mut u32, b);

            // Write the control register.
            core::ptr::write_volatile((Self::BASE + CONTROL) as *mut u32, ((Coprocessor::Matrix as u32) << 4) | o as u32);

            // Write the Length register.
            core::ptr::write_volatile((Self::BASE + LENGTH) as *mut u32, l);

            #[cfg(feature = "defmt")]
            {
                use core::ptr::read_volatile as read;

                // Log the addresses of the matrices.
                defmt::debug!("Matrix Operation @ {:#X} | {:#X} | {:#X}", read(0x500A6010 as *const u32), read(0x500A6018 as *const u32), read(0x500A6000 as *const u32));
    
                // Log the length register.
                defmt::debug!("Matrix length A: [{0=0..8}x{0=8..16}] B: [{0=16..24}]", read(0x500A6104 as *const u32));
            }
        }
    }
}

impl super::sleep::Sleep for Engine {
    fn create() -> Self {
        Self
    }
}

/*
impl Drop for Engine {
    fn drop(&mut self) {
        // Increase the POWEROFF counter to allow the other interfaces to sleep.
        super::poweroff();
    }
}
*/


pub enum Coprocessor {
    Matrix = 1
}



/// Internal list of checks for operations with 3 matrices.
#[derive(Clone, Copy, Eq, PartialEq)]
enum MatrixCheck3 {
    /// Checks for equal dimensions.
    Equal,

    /// Checks for multiplication compatible dimensions.
    Multiply,
}


#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MatrixError {
    IncompatibleDimensions,
}