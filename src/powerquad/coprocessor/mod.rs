//! PowerQuad Coprocessor interface.



pub mod traits;

mod asm;
mod engine;
mod function;



use traits::{
    CoprocessorInterface, CoprocessorNumber,
    CoprocessorOperation, ReadType,
};



/// PowerQuad Coprocessor interface.
pub struct Coprocessor<const N: usize>;

impl<const N: usize> Coprocessor<N> {
    /// Internal function to create a new instance.
    pub(super) fn create() -> Self {
        Self
    }

    /// Turns off the PowerQuad coprocessor (if all other PQ interfaces are off).
    pub fn sleep(self) -> SleepToken<N> {
        // Drop the interface to force a power off.
        core::mem::drop(self);

        // Create the sleep token.
        SleepToken::create()
    }
}

impl<'a, const N: usize> CoprocessorInterface<'a, f32> for Coprocessor<N> where Coprocessor<N>: 'a {
    const CPID: usize = N << 1;

    type Result = Operation<'a, N, f32>;

    fn result(&'a mut self, read: ReadType) -> Operation<'a, N, f32> {
        Operation::create( read, self )
    }
}

impl<const N: usize> Drop for Coprocessor<N> {
    fn drop(&mut self) {
        // Increase the POWEROFF counter to allow the other interfaces to sleep.
        super::poweroff();
    }
}



/// An ongoing PowerQuad Coprocessor operation.
pub struct Operation<'a, const N: usize, NUMBER: CoprocessorNumber> {
    /// The read function to use.
    read: ReadType,

    /// This is a mutability lock on the origin coprocessor.
    _lock: &'a Coprocessor<N>,

    _phantom: core::marker::PhantomData<NUMBER>,
}

impl<'a, const N: usize, NUMBER: CoprocessorNumber> Operation<'a, N, NUMBER> {
    /// Internal functino to create an ongoing operation.
    pub(self) fn create( read: ReadType, _lock: &'a Coprocessor<N> ) -> Self {
        Self { _lock, read, _phantom: core::marker::PhantomData, }
    }

    /// Internal read function of type MUL.
    fn read<const READ: u32, const TYPE: usize>(self) -> NUMBER {
        // Read the result raw.
        let raw = asm::mrc::<{traits::PQID}, READ, TYPE, 0, 0>();

        // Transform into the type.
        let result = NUMBER::from( raw );

        result
    }
}

impl<'a> CoprocessorOperation<f32> for Operation<'a, 0, f32> {
    const CPID: usize = 0 << 1;

    fn finish(self) -> f32 {
        match self.read {
            ReadType::Add => self.read::<1, {0 | (0 << 1)}>(),
            ReadType::Mul => self.read::<0, {0 | (0 << 1)}>(),
        }
    }
}

impl<'a> CoprocessorOperation<f32> for Operation<'a, 1, f32> {
    const CPID: usize = 1 << 1;

    fn finish(self) -> f32 {
        match self.read {
            ReadType::Add => self.read::<1, {0 | (1 << 1)}>(),
            ReadType::Mul => self.read::<0, {0 | (1 << 1)}>(),
        }
    }
}



impl CoprocessorNumber for f32 {
    const ID: usize = 0;

    fn raw(self) -> u32 {
        unsafe { core::mem::transmute(self) }
    }

    fn from(x: u32) -> Self {
        unsafe { core::mem::transmute(x) }
    }
}



/// Sleep token to power on the PowerQuad.
pub struct SleepToken<const N: usize>;

impl<const N: usize> SleepToken<N> {
    /// Internal function to create a new instance.
    pub(super) fn create() -> Self {
        Self
    }

    /// Turns on the PowerQuad.
    pub fn wake(self) -> Coprocessor<N> {
        // Power on the PowerQuad.
        super::poweron();

        // Create the sleep token.
        Coprocessor::create()
    }
}
