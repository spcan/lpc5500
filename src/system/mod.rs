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
    const SYSCON: u32 = 0x40000000;

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

    // Initialize all memory regions.
    unsafe {
        core::arch::asm!(
            // Push the registers that will be used. 
            "push {{r0-r3}}",

            // Zero out .bss.
            "
            ldr r0, =__sbss
            ldr r1, =__ebss
            movs r2, #0

            2:
            cmp r1, r0
            beq 3f
            stm r0!, {{r2}}
            b 2b

            3:
            ",

            // Initialize .data.
            "
            ldr r0, =__sdata
            ldr r1, =__edata
            ldr r2, =__sidata

            4:
            cmp r1, r0
            beq 5f
            ldm r2!, {{r3}}
            stm r0!, {{r3}}
            b 4b

            5:
            ",

            // Pop the registers used.
            "pop {{r0-r3}}",

            options(nostack)
        );
    }

    // Initialize the clock system.
    clocks::init();

    // Initialize the user interface.
    let user = user::UserSystemControl::init();

    // Enable the PowerQuad (CP0) and CASPER (CP1) coprocessor interfaces.
    unsafe {
        // Experimental assembly implementation for code compactness.
        /*
        core::arch::asm!(
            // Push the registers that will be used. 
            "push {{r0-r2}}",

            // Load the base address on the r0 register.
            "ldr r0, =0xE000ED88",

            // Load the CPACR and NSACR registers onto R1 and R2.
            "ldmia r0, {{r1,r2}}",

            // Modify R1 and R2 to enable the registers.
            "orr r1,r1,#0x0000000F",
            "orr r2,r2,#0x00000003",

            // Write back the new registers.
            "stmia r0, {{r1,r2}}",

            // Pop the registers used.
            "pop {{r0-r2}}",

            options(nostack)
        );
        */
        // Base address of the CPACR.
        const BASE: u32 = 0xE000ED88;

        // Read the CPACR @ 0xE000ED88 and NSACR @0xE000ED8C.
        let mut cpacr = core::ptr::read_volatile((BASE + 0) as *const u32);
        let mut nsacr = core::ptr::read_volatile((BASE + 4) as *const u32);

        // Enable full access for the PQ and CASPER.
        cpacr |= (0b11 << 2) | (0b11 << 0);
        nsacr |= (   1 << 1) | (   1 << 0);

        // Write the modified CPACR and NSACR.
        core::ptr::write_volatile((BASE + 0) as *mut u32, cpacr);
        core::ptr::write_volatile((BASE + 4) as *mut u32, nsacr);
    }

    // Return the `UserSystemControl` instance.
    return user;
}
