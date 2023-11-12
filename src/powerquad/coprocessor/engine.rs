//! Namespaced Coprocessor Engine IDs for compile time instructions.



pub(super) struct Engine;

impl Engine {
    /// Transcendental Function engine ID.
    pub(super) const TRANS: usize = 0;

    /// Trigonometric Function engine ID.
    pub(super) const TRIG: usize = 1;

    /// Biquad Function engine ID.
    pub(super) const BIQUAD: usize = 2;
}
