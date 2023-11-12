//! System control module.



pub(crate) mod clocks;
pub(crate) mod enable;
pub(crate) mod flash;
pub(crate) mod pdrun;
pub(crate) mod power;
pub(crate) mod reset;

pub mod user;



/// Driver to control main system functions (enable, reset, etc...).
pub struct SystemControl;

// Metadata, constants, addresses and other necessary data.
impl SystemControl {
    /// Base address of the SYSCON peripheral.
    const SYSCON: u32 = 0x50000000;

    /// Offset of the RSTSET register block.
    const RSTSET: u32 = 0x120;

    /// Offset of the RSTCLR register block.
    const RSTCLR: u32 = 0x140;

    /// Offset of the AHBSET register block.
    const AHBSET: u32 = 0x220;

    /// Offset of the AHBCLR register block.
    const AHBCLR: u32 = 0x240;
}


// Peripheral control. Reset, enable, etc...
impl SystemControl {
    /// Enables the clock to a peripheral.
    pub(crate) fn enable<P: enable::Enable>() {
        // The reset clear register.
        let dst = (Self::SYSCON + Self::AHBSET) + (P::REG * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFF);
        }
    }

    /// Disables the clock to a peripheral.
    pub(crate) fn disable<P: enable::Disable>() {
        // The reset clear register.
        let dst = (Self::SYSCON + Self::AHBCLR) + (P::REG * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFF);
        }
    }

    /// Puts a peripheral in the reset state.
    pub(crate) fn reset<P: reset::Reset>() {
        // The reset clear register.
        let dst = (Self::SYSCON + Self::RSTSET) + (P::REG * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFF);
        }
    }

    /// Takes a peripheral out of the reset state.
    pub(crate) fn unreset<P: reset::Unreset>() {
        // The reset clear register.
        let dst = (Self::SYSCON + Self::RSTCLR) + (P::REG * 4);

        unsafe {
            // Write to the clear register.
            core::ptr::write_volatile(dst as *mut u32, 1 << P::OFF);
        }
    }
}



/// Common trait for all peripherals that can be controlled from SYSCON.
pub(crate) trait Control {
    /// Index of the register inside the register block.
    const REG: u32;

    /// Index of the control bit inside the register.
    const OFF: u8;
}


/// Initializes the system to its most basic functional state.
pub fn init() -> user::UserSystemControl {
    // Unreset all memory regions.
    {}

    // Initialize the clock system.
    clocks::init();

    // Initialize the user interface.
    let user = user::UserSystemControl::init();

    // Initialize the analog control peripheral.
    //reset::ResetControl::unreset::<analog::AnalogControl>();
    //enable::EnableControl::enable::<analog::AnalogControl>();

    // Initialize the clocks.
    //clocks::init();

    user
}
