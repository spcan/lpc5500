//! Internal oscillator module.
//! Controls all internal free running oscillators.



mod impls;
mod info;
mod traits;



pub use impls::*;
pub use info::ClockInformation;



/// Internal driver of the FROs.
pub(super) struct Control;

impl Control {
    /// Initializes the internal oscillators.
    pub(super) fn init() {
        use crate::system::{
            enable::Enable,
            pdrun::PowerOn,
            reset::Unreset,
        };

        // Create an instance of `Self` to allow modifications.
        let mut ctrl = Self;

        // Unreset and enable the clock to the ANACTRL peripheral.
        ctrl.unreset();

        // Enable the clock to the CTRL.
        ctrl.enable();

        // Power on the control.
        ctrl.poweron();

        // Enable the 12 MHz clock.
        Self::enable::<FRO12MHz>();

        // Enable the 48 MHz clock.
        Self::enable::<FRO48MHz>();

        // Enable the 96 MHz clock.
        Self::enable::<FRO96MHz>();
    }

    /// Enables the given FRO clock signal.
    pub(self) fn enable<S: traits::FROEnable>() {
        unsafe {
            // Address of the CTRL register [ANACTRL->FRO192M_CTRL @ 0x50013000 + 0x10].
            const ADDRESS: usize = 0x50013000 + 0x10;

            // Read the CTRL register .
            let ctrl = core::ptr::read_volatile( ADDRESS as *const u32 );

            // Write the modified CTRL register.
            core::ptr::write_volatile(ADDRESS as *mut u32, ctrl | (1 << S::OFFSET));
        }
    }

    /// Disables the given FRO clock signal.
    pub(self) fn disable<S: traits::FRODisable>() {
        unsafe {
            // Address of the CTRL register [ANACTRL->FRO192M_CTRL @ 0x50013000 + 0x10].
            const ADDRESS: usize = 0x50013000 + 0x10;

            // Read the CTRL register .
            let ctrl = core::ptr::read_volatile( ADDRESS as *const u32 );

            // Write the modified CTRL register.
            core::ptr::write_volatile(ADDRESS as *mut u32, ctrl & !(1 << S::OFFSET));
        }
    }
}

impl crate::system::enable::Enable for Control {
    const REGISTER : usize = 2;
    const OFFSET : usize = 27;
}

impl crate::system::reset::Unreset for Control {
    const REGISTER : usize = 2;
    const OFFSET : usize = 27;
}

impl crate::system::pdrun::PowerOn for Control {
    const OFFSET: u8 = 5;
}
