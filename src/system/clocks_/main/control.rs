//! A collection of the main clock controls.



pub struct ClockControl {
    /// The Main Selector A clock signal control.
    pub maina: super::maina::MainSelectA,

    /// The Main Selector B clock signal control.
    pub mainb: super::mainb::MainSelectB,

    /// The Main (AHB) clock signal control.
    pub main: super::Main,
}

impl ClockControl {
    /// Static initializer.
    pub(crate) const fn new() -> Self {
        Self {
            maina: super::maina::MainSelectA::new(),
            mainb: super::mainb::MainSelectB::new(),
            main: super::Main::new(),
        }
    }
}
