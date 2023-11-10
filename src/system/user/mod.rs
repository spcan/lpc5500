//! User accessible system configuration.



pub mod flash;

pub mod main;



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};

use crate::system::power::PowerControl;



/// This control abstraction includes all system configurations that can be
/// modified on the fly by the user.
pub struct UserSystemControl {
    /// Flash control.
    /// Allows the user to modify Flash acceleration.
    pub flash: flash::Flash,

    // PLL1 control.
    // Allows the user to change the configuration of the PLL1.
    // PLL1 is reserved to be used as a source for the main clock.
    //

    /// Main clock control.
    main: main::MainClock,
}

impl UserSystemControl {
    /// PMC base address.
    const PMC: u32 = 0x40020000;

    /// DCDC 0 register offset.
    const DCDC0: u32 = 0x10;

    /// DCDC 1 register offset.
    const DCDC1: u32 = 0x14;

    /// LDOPMU register offset.
    const LDOPMU: u32 = 0x1C;

    /// Sets the clock source of the main clock.
    pub fn setclock(&mut self, source: main::ClockSource) {
        // Get the current frequency and clock source.
        let (cfreq, src) = self.main.getclock();

        // Skip changes if the requested source is the current source.
        if src == source { return }

        // Get the target frequency.
        let tfreq = source.frequency();

        // If the target frequency is higher than the current frequency configure the device.
        if tfreq > cfreq {
            // Configure the voltage necessary for the target frequency.
            PowerControl::configfreq(tfreq);

            // Configure the Flash wait states.
            self.flash.configfreq(tfreq);
        }

        // If the target frequency is lower than the current frequency configure the device.
        if tfreq < cfreq {
            // Configure the voltage necessary for the target frequency.
            PowerControl::configfreq(tfreq);

            // Configure the Flash wait states.
            self.flash.configfreq(tfreq);
        }
    }
}
