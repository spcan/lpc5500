//! List of all PowerQuad matrix operations.



pub enum MatrixOperation {
    /// Scales the given matrix.
    Scale = 1,

    /// Multiplies two matrices.
    Multiply = 2,

    /// Adds two matrices.
    Add = 3,

    /// Inverts a matrix.
    Inverse = 4,

    /// Performs the Hadamard product of two matrices.
    Product = 5,

    /// Subtracts two matrices.
    Subtract = 7,

    /// Transposes a matrix.
    Transpose = 10,
}
