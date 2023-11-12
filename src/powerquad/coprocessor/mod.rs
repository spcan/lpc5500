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
}

impl<const N: usize> CoprocessorInterface<f32> for Coprocessor<N> {
    const CPID: usize = N << 1;

    type Result = Operation<N, f32>;

    fn result(self, read: ReadType) -> Operation<N, f32> {
        Operation::create( read )
    }
}



/// An ongoing PowerQuad Coprocessor operation.
pub struct Operation<const N: usize, NUMBER: CoprocessorNumber> {
    /// The read functino to use.
    read: ReadType,

    _phantom: core::marker::PhantomData<NUMBER>,
}

impl<const N: usize, NUMBER: CoprocessorNumber> Operation<N, NUMBER> {
    /// Internal functino to create an ongoing operation.
    pub(self) fn create( read: ReadType ) -> Self {
        Self { read, _phantom: core::marker::PhantomData, }
    }

    /// Internal read function of type MUL.
    fn read<const READ: u32, const TYPE: usize>(self) -> (NUMBER, Coprocessor<N>) {
        // Read the result raw.
        let raw = asm::mrc::<{traits::PQID}, READ, TYPE, 0, 0>();

        // Transform into the type.
        let result = NUMBER::from( raw );

        ( result, Coprocessor::create() )
    }
}

impl CoprocessorOperation<f32> for Operation<0, f32> {
    const CPID: usize = 0 << 1;

    type Origin = Coprocessor<0>;

    fn finish(self) -> (f32, Self::Origin) {
        match self.read {
            ReadType::Add => self.read::<1, {0 | (0 << 1)}>(),
            ReadType::Mul => self.read::<0, {0 | (0 << 1)}>(),
        }
    }
}

impl CoprocessorOperation<f32> for Operation<1, f32> {
    const CPID: usize = 1 << 1;

    type Origin = Coprocessor<1>;

    fn finish(self) -> (f32, Self::Origin) {
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
