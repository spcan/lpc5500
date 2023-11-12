//! User accessible system configuration.



pub use crate::system::{
    clocks::main::ClockSource,
    flash::{
        Acceleration, BufferUsage,
    },
};

use crate::system::{
    clocks::main::MainClock,
    flash::FlashControl,
    power::PowerControl
};



/// This control abstraction includes all system configurations that can be
/// modified on the fly by the user.
pub struct UserSystemControl {
    /// Flash control.
    pub flash: FlashControl,

    /// Main clock control.
    pub main: MainClock,

    /// System Power control.
    pub power: PowerControl,
}

impl UserSystemControl {
    /// SYSCON base address.
    const SYSCON: u32 = 0x50000000;

    /// Main Clock source select A offset.
    const MAINSELA: u32 = 0x280;

    /// Main Clock source select B offset.
    const MAINSELB: u32 = 0x284;

    /// Initialize the user system control interface.
    pub(super) fn init() -> Self {
        // Initialize flash.
        let flash = FlashControl::init();

        // Initialize the main clock.
        let main = MainClock::init();

        // Initialize power control.
        let power = PowerControl::init();

        Self { flash, main, power, }
    }

    /// Sets the clock source of the main clock.
    pub fn setclock(&mut self, source: ClockSource) {
        // Get the current frequency and clock source.
        let (cfreq, current) = self.main.getclock();

        // Skip changes if the requested source is the current source.
        if current == source { return }

        // Get the target frequency.
        let tfreq = source.frequency();

        //defmt::debug!("Switching main clock from {} [{} MHz] to {} [{} MHz]", current, cfreq / 1_000_000, source, tfreq / 1_000_000);

        // If the target frequency is higher than the current frequency configure the device.
        if tfreq > cfreq {
            // Configure the voltage necessary for the target frequency.
            self.power.target(tfreq);

            // Configure the Flash wait states.
            self.flash.target(tfreq);
        }

        // Switch the main clock.
        self.main.switch( source );

        // If the target frequency is lower than the current frequency configure the device.
        if tfreq < cfreq {
            // Configure the voltage necessary for the target frequency.
            self.power.target(tfreq);

            // Configure the Flash wait states.
            self.flash.target(tfreq);
        }
    }
}
