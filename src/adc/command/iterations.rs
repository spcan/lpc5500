//! Defines the automatic repetition of a command.



#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Iterations {
    /// Number of iterations this command is executed.
    count: u8,

    /// Indicates if the channel index is incremented on each iteration.
    increment: bool,
}

impl Iterations {
    /// Creates an `Iterations` configuration with the given parameters.
    /// `count` will be clamped to the range 0:15.
    /// <div class="warning">WARNING : There are invalida</div>
    pub const fn create(count: u8, increment: bool) -> Self {
        Self { count: count.clamp(0, 15), increment, }
    }

    /// Creates the default `Iterations` configuration.
    pub const fn new() -> Self {
        Self { count: 0, increment: false, }
    }
}
