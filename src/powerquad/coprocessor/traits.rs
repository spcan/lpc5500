//! Traits used to interface with the PowerQuad coprocessor.



use super::engine::Engine;
use super::function::*;



/// PowerQuad coprocessor ID.
pub(super) const PQID: u32 = 0;



/// Common trait for all number types allowed by the PowerQuad coprocessor.
/// Thses types are `f32` and `fx32`.
pub trait CoprocessorNumber: Copy + Sized {
    /// The type ID in the coprocessor interface.
    /// 0 for Float, 1 for Fixed.
    const ID: usize;

    /// Transforms the type into its raw value.
    fn raw(self) -> u32;

    /// Creates the type from its raw value.
    fn from(raw: u32) -> Self;
}



/// Common trait for the two coprocessor interfaces of the PowerQuad.
pub trait CoprocessorInterface<'a, NUMBER: CoprocessorNumber>: Sized {
    /// The ID of the sub-coprocessor to target.
    /// (0 << 1) for CP0 and (1 << 1) for CP1.
    const CPID: usize;

    /// The type returned by operations with this coprocessor.
    type Result;

    /// Initiates the calculation of the inverse of the given number.
    #[inline(always)]
    fn inv(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::INV}>(x.raw());

        // Create the result.
        self.result( ReadType::Mul )
    }

    /// Initiates the calculation of the natural logarithm of the given number.
    #[inline(always)]
    fn ln(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::LN}>(x.raw());

        // Create the result.
        self.result( ReadType::Add )
    }

    /// Initiates the calculation of the square root of the given number.
    #[inline(always)]
    fn sqrt(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::SQRT}>(x.raw());

        // Create the result.
        self.result( ReadType::Mul )
    }

    /// Initiates the calculation of the inverse square root of the given number.
    #[inline(always)]
    fn isqrt(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::ISQRT}>(x.raw());

        // Create the result.
        self.result( ReadType::Mul )
    }

    /// Initiates the calculation of the exponential of the given number.
    #[inline(always)]
    fn exp(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::EXP}>(x.raw());

        // Create the result.
        self.result( ReadType::Mul )
    }

    /// Initiates the calculation of the exponential negative of the given number.
    #[inline(always)]
    fn expn(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRANS + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRANS}, {TransFns::EXPN}>(x.raw());

        // Create the result.
        self.result( ReadType::Mul )
    }

    /// Initiates the calculation of the sine of the given number.
    #[inline(always)]
    fn sin(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRIG + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRIG}, {TrigFns::SIN}>(x.raw());

        // Create the result.
        self.result( ReadType::Add )
    }

    /// Initiates the calculation of the cosine of the given number.
    #[inline(always)]
    fn cos(&'a mut self, x: NUMBER) -> Self::Result
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); Engine::TRIG + (4 * NUMBER::ID)]: Sized
    {
        // Send the parameters to the coprocessor.
        Self::mcr::<{Engine::TRIG}, {TrigFns::COS}>(x.raw());

        // Create the result.
        self.result( ReadType::Add )
    }

    /// Low-level function to call the MCR instruction.
    #[inline(always)]
    fn mcr<const ENGINE: usize, const FUNCTION: u32>(x: u32)
        where [(); NUMBER::ID | Self::CPID]: Sized, [(); ENGINE + (4 * NUMBER::ID)]: Sized
    {
        super::asm::mcr::<PQID, FUNCTION, {NUMBER::ID | Self::CPID}, 0, {ENGINE + (4 * NUMBER::ID)}>(x);
    }

    /// Internal function to create a result.
    fn result(&'a mut self, read: ReadType) -> Self::Result;
}



/// Common trait for all ongoing coprocessor operations.
pub trait CoprocessorOperation<NUMBER: CoprocessorNumber>: Sized {
    /// The ID of the sub-coprocessor to target.
    /// (0 << 1) for CP0 and (1 << 1) for CP1.
    const CPID: usize;

    /// Blocks until the end of the operation, returning the result of the operation and the coprocessor interface.
    fn finish(self) -> NUMBER;

    /// Discards the operation result, releasing the lock on the coprocessor.
    fn discard(self) {
        core::mem::drop(self)
    }
}



/// NXP has the reads from the coprocessor divided in Mul and Add types.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ReadType {
    Add,

    Mul,
}