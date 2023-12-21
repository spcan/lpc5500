


mod format;
mod ops;
mod typed;


pub(self) use format::*;

pub(super) use ops::MatrixOperation;

pub use typed::Matrix as TypedMatrix;

use core::ops::{ Index, IndexMut, };



/// Common trait for all matrix types.
/// All `MatrixTrait` implementors can have their elements accessed by 
pub trait MatrixTrait: IndexRow + IndexElement where {
    /// Returns the address of the data buffer.
    fn address(&self) -> u32;

    /// Returns the dimensions of the matrix (columns, rows).
    fn dims(&self) -> (usize, usize);
}



/// Internal trait that forces the matrix to be row addressable.
trait IndexRow: Index<usize, Output=[f32]> + IndexMut<usize, Output=[f32]> {}



/// Internal trait that forces the matrix to be element addressable.
trait IndexElement: Index<(usize, usize), Output=f32> + IndexMut<(usize, usize), Output=f32> {}
