//! Common abstractions of the ADC peripheral.



mod average;
mod compare;
mod mode;
mod precision;
mod resolution;
mod time;



pub use average::Average;
pub use compare::Compare;
pub use mode::Mode;
pub use precision::Precision;
pub use resolution::Resolution;
pub use time::SampleTime;
