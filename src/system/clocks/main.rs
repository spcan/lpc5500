//! Main clock control.
//! This interface is available to the user to implement dynamic frequency scaling.







#[link_section = ".bss.CLOCKS.main"]
static mut FREQUENCY: u32 = 0;



/// User facing clock interface.
pub struct MainClock {
    /// Current frequency source.
    source: Source,
}

impl MainClock {
    /// Initializes the `MainClock` interface.
    pub fn init() -> Self {
        // Check which source is currently selected.
    }
}




/// All possible main clock sources.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Source {
    /// Internal FRO at 1 MHz.
    FRO1MHz,

    /// Internal FRO at 12 MHz.
    FRO12MHz,

    /// Internal FRO at 96 MHz.
    FRO96MHz,

    /// Internal PLL 1.
    PLL1,
}
