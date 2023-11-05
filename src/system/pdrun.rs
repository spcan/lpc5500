//! Control over the PD RUN register.




pub(crate) struct PDControl;

impl PDControl {
    /// Function to power on a peripheral.
    pub(crate) fn on<P: PowerOn>() {
        // Create the pointer to the register [PMC->PDRUNCRGCLR(X) @ 0x40020000 + 0xC8].
        let dst = 0x40020000 + 0xC8;

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFFSET);
        }
    }

    /// Function to power off a peripheral.
    pub(crate) fn off<P: PowerOff>() {
        // Create the pointer to the register [PMC->PDRUNCRGSET(X) @ 0x40020000 + 0xC0].
        let dst = 0x40020000 + 0xC0;

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFFSET);
        }
    }
}



/// Common trait for all peripherals that can be powered on.
pub trait PowerOn: Sized {
    /// The in-register offset of the control bit.
    const OFFSET: u8;

    /// Takes the peripheral out of the reset state.
    fn poweron(&mut self) {
        PDControl::on::<Self>()
    }
}



/// Common trait for all peripherals that can be powered off.
pub trait PowerOff: PowerOn {
    /// Takes the peripheral out of the reset state.
    fn poweroff(&mut self) {
        PDControl::off::<Self>()
    }
}
