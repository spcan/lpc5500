//! Main clock control.
//! This interface is available to the user to implement dynamic frequency scaling.







#[link_section = ".bss.LPC5500.clocks.MAIN"]
static mut FREQUENCY: u32 = 0;



/// User facing clock interface.
pub struct MainClock {
    /// Current frequency source.
    source: Source,

    // PLL1 control.
    //pll1: ,
}

impl MainClock {
    /// Switches the main clock signal source.
    pub fn switch(&mut self, source: Source) {
        // Get the current frequency.
        let cfreq = source.frequency();

        // Get the target frequency.
        let tfreq = self.source.frequency();

        // If the target frequency is higher than the current frequency, increase Flash wait states.

        // If the target frequency is lower than the current frequency, decrease Flash wait states.
    }

    /// Configures the wait states of the Flash for the desired freqency.
    fn waitstates() {
        
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

    // Internal PLL 1.
    //PLL1,
}
