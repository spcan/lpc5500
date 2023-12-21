//! An ADC command that will be executed after a trigger.



mod conversion;
mod iterations;



pub use conversion::*;
pub use iteration::*;



pub struct Command {
    /// The conversion configuration to execute
    conversion: Conversion,

    /// Configures if the buffer executes repeatedly.
    iter: Iterations,
}

impl Command {
    /// Creates an ADC command with the given configurations.
    pub const fn create(conversion: Conversion, iter: Iterations) -> Self {
        Self { conversion, iter, }
    }
}
