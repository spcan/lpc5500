//! Const Typed Matrix (2D Tensor) implementation.
//! This implementation is const generic over the dimensions of the matrix.
//! 
//! The memory layout is row major (due to hardware constraints).



#[repr(C)]
pub struct Matrix<const C: usize, const R: usize>(pub(self) [f32; C * R]) where [(); C * R]: Sized;

impl<const C: usize, const R: usize> Matrix<C, R> where [(); C * R]: Sized {
    /// Creates a matrix of zeroes.
    pub const fn zeroes() -> Self {
        Self([0.0; C * R])
    }

    /// Calculates the offset of a row.
    fn offset(&self, index: usize) -> usize {
        // Assert the validity of the row index.
        if index > R { panic!("Index {} is bigger than matrix row number {}", index, R) }

        // Get the base pointer.
        let base = self as *const _ as usize;

        // Add the necessary offset.
        base + (C * index * core::mem::size_of::<f32>())
    }
}

impl<const C: usize> Matrix<C, C> where [(); C * C]: Sized {
    /// Creates an identity matrix.
    pub const fn eye() -> Self {
        // Create the zeroed buffer.
        let mut data = [0.0; C * C];

        // Mutable index for the const loop.
        let mut i = 0;

        // Set the diagonal to ones.
        loop {
            // Break if reached end of loop.
            if i >= C { break; }

            // Set the diagonal one.
            data[(i * C) + i] = 1.0;

            // Increment index.
            i += 1;
        }

        Self(data)
    }
}


impl<const C: usize, const R: usize> super::MatrixTrait for Matrix<C, R> where [(); C * R]: Sized {
    fn address(&self) -> u32 {
        self as *const _ as u32
    }

    fn dims(&self) -> (usize, usize) {
        (C, R)
    }
}

impl<const C: usize, const R: usize> core::ops::Index<usize> for Matrix<C, R> where [(); C * R]: Sized {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { core::slice::from_raw_parts(self.offset(index) as *const _, C) }
    }
}

impl<const C: usize, const R: usize> core::ops::IndexMut<usize> for Matrix<C, R> where [(); C * R]: Sized {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { core::slice::from_raw_parts_mut(self.offset(index) as *mut _, C) }
    }
}

impl<const C: usize, const R: usize> core::ops::Index<(usize, usize)> for Matrix<C, R> where [(); C * R]: Sized {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[((index.0 * C) + index.1) * core::mem::size_of::<f32>()]
    }
}

impl<const C: usize, const R: usize> core::ops::IndexMut<(usize, usize)> for Matrix<C, R> where [(); C * R]: Sized {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[((index.0 * C) + index.1) * core::mem::size_of::<f32>()]
    }
}

#[cfg(feature = "defmt")]
impl<const C: usize, const R: usize> defmt::Format for Matrix<C, R> where [(); C * R]: Sized {
    fn format(&self, f: defmt::Formatter) {
        super::mtxformat(&self.0[..], C, R, true, f)
    }
}

impl<const C: usize, const R: usize> super::IndexElement for Matrix<C, R> where [(); C * R]: Sized {}
impl<const C: usize, const R: usize> super::IndexRow     for Matrix<C, R> where [(); C * R]: Sized {}
