//! Peripheral reset control.



/// Common trait for all peripheral that can be enabled.
pub(crate) trait Enable: super::Control {}

/// Common trait for all peripheral that can be disabled.
/// WARNING : Not all peripherals can be disabled.
pub(crate) unsafe trait Disable: Enable {}
