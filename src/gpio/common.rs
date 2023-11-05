//! Common abstractions of GPIO devices.



/// Pull resistor configurations.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PullMode {
    /// No pull resistor enabled.
    None = 0x00,

    /// Pull-Down resistor enabled.
    PullDown = 0x01,

    /// Pull-Up resistor enabled.
    PullUp = 0x02,

    /// Repeater mode pulls to the last output level.
    Repeater = 0x03,
}


/// Slew rate configurations.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SlewRate {
    /// Slow slew rate. Allows for more outputs to be switched simultaneously.
    Slow = 0,

    /// Fast slew rate.
    Fast = 1,
}



/// Input polarity configurations.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Polarity {
    /// Normal polarity.
    Normal = 0,

    /// Inverted polarity.
    Inverted = 1,
}



/// Input polarity configurations.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GPIOMode {
    /// Push-Pull mode.
    PushPull = 0,

    /// Open-Drain mode.
    OpenDrain = 1,
}
