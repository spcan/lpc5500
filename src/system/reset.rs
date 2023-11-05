//! Peripheral reset control.



/// Common trait for all peripheral that can be taken out of reset.
pub(crate) trait Unreset: super::Control {}



/// Common trait for all peripherals that can be put in reset.
/// WARNING : Not all peripherals can be reset.
pub(crate) unsafe trait Reset: Unreset {}
