//! Main clock control.
//! This clock feeds the CPUs, AHB bus, APB bridge, ADC and USBs.




/// Main clock first selector.
/// In the documentation corresponds with the Main Clock Selector A.
pub struct MainClockSelector {
    /// The current source of the Main Clock Selector A.
    source: Source,
}


/// Common trait for all clock sources of the MainClockSelector.
pub trait MainClockSelectorSource {
    
}









pub struct MainClock {
    /// Currently selected source clock.
    source: Source,
}

impl MainClock {
    /// Derives the output of the `MainClock` from the given source.
    pub fn from<S: MClockSource>(&mut self) {
        // Set the 
        unsafe { core::ptr::write_volatile((0x50000000 + 0x284) as *mut u32, S::BITS); }
    }
}

/// Common trait for all clock sources that can be selected as `MainClock` sources.
pub trait MClockSource {
    /// The bit value to write to the register (0 - 3).
    const BITS: u32;
}


/// All possible sources for the `MainClock`.
pub enum Source {
    /// Main Clock Selector A clock signal.
    MainClockSelA,

    /// PLL 0 clock signal.
    PLL0,

    /// PLL 1 clock signal.
    PLL1,

    /// 32 kHz Oscillator clock signal.
    Osc32K,
}
