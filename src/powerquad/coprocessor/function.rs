//! Namespaced Function IDs for compile time instructions.




/// Transcendental functions.
pub(super) struct TransFns;

impl TransFns {
    /// Computes the Inverse of a number (1/x).
    pub(super) const INV: u32 = 0;

    /// Computes the Natural Logarithm of a number.
    pub(super) const LN: u32 = 1;

    /// Computes the Square Root of a number.
    pub(super) const SQRT: u32 = 2;

    /// Computes the Inverse Square Root of a number.
    pub(super) const ISQRT: u32 = 3;

    /// Computes the Exponential of a number.
    pub(super) const EXP: u32 = 4;

    /// Computes the Exponential Negative of a number.
    pub(super) const EXPN: u32 = 5;

    /// Computes the division between two numbers.
    pub(super) const DIV: u32 = 6;
}



/// Trigonometric functions.
pub(super) struct TrigFns;

impl TrigFns {
    /// Computes the sine of a number.
    pub(super) const SIN: u32 = 0;

    /// Computes the cosine of a number.
    pub(super) const COS: u32 = 1;
}
