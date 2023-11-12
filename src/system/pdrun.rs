//! Control over the PD RUN register.




pub(crate) struct PDControl;

impl PDControl {
    /// Base address of the PMC peripheral.
    const PMC: u32 = 0x40020000;

    /// Offset of the PDRUNSET peripheral.
    const PDRUNSET: u32 = 0xC0;

    /// Offset of the PDRUNCLR peripheral.
    const PDRUNCLR: u32 = 0xC8;

    /// Function to power on a peripheral.
    #[inline(never)]
    pub(crate) fn on(offset: u8) {
        // Create the pointer to the register [PMC->PDRUNCRGCLR(X) @ 0x40020000 + 0xC8].
        let dst = (Self::PMC + Self::PDRUNCLR) as *mut u32;

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst, 1 << offset);
        }
    }

    /// Function to power off a peripheral.
    #[inline(never)]
    pub(crate) fn off(offset: u8) {
        // Create the pointer to the register [PMC->PDRUNCRGSET(X) @ 0x40020000 + 0xC0].
        let dst = (Self::PMC + Self::PDRUNSET) as *mut u32;

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst, 1 << offset);
        }
    }
}



/// Common trait for all peripherals that can be powered on.
pub trait PowerOn: Sized {
    /// The in-register offset of the control bit.
    const OFFSET: u8;

    /// Takes the peripheral out of the reset state.
    fn poweron(&mut self) {
        PDControl::on( Self::OFFSET )
    }
}



/// Common trait for all peripherals that can be powered off.
pub trait PowerOff: PowerOn {
    /// Takes the peripheral out of the reset state.
    fn poweroff(&mut self) {
        PDControl::off( Self::OFFSET )
    }
}
