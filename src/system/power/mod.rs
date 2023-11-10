//! Power Control module.



pub mod voltages;



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};



pub struct PowerControl;

impl PowerControl {
    /// PMC base address.
    const PMC: u32 = 0x40020000;

    /// DCDC 0 register offset.
    const DCDC0: u32 = 0x10;

    /// DCDC 1 register offset.
    const DCDC1: u32 = 0x14;

    /// LDOPMU register offset.
    const LDOPMU: u32 = 0x1C;

    /// Configures the power of the device for the given frequency.
    pub(crate) fn configfreq(freq: u32) {
        // Get the corresponding power profile.
        let profile = PowerProfile::from( freq );

        // Get the NMPA DCDC values.
        let (dcdc0, dcdc1) = profile.nmpa();

        // Write the DCDC values to the PMC.
        unsafe {
            write( (Self::PMC + Self::DCDC0) as *mut u32, dcdc0 );
            write( (Self::PMC + Self::DCDC1) as *mut u32, dcdc1 );
        }

        // Get the process node.
        let process = ProcessNode::get();

        // Set the core voltage for the given process node and profile.
        self.setvoltage( process.voltage(profile) );
    }

    /// Sets the voltage of the device.
    pub(crate) fn setvoltage(voltage: u32) {
        use voltages::*;

        // Get the core voltage.
        let core = CoreVoltage::from(voltage);

        // Get the LDO AO and LDO AO Boost voltages.
        let (ldoao, boost) = AOVoltage::from(core);

        // Setup Always-On domain LDO.
        unsafe {
            // Get the pointer to the register.
            let ptr = Self::PMC + Self::LDOPMU;

            // Read the register.
            let mut r = read(ptr as *const u32);

            // Clear the values and set the voltage.
            r &= !((0x1F << 10) | 0x1F);
            r |= ((boost as u32) << 10) | (ldoao as u32);

            // Write the modified value.
            write(ptr as *mut u32, r);
        }

        // Setup core voltage.
        unsafe {
            // Get the pointer to the register.
            let ptr = Self::PMC + Self::DCDC0;

            // Read the register.
            let mut r = read(ptr as *const u32);

            // Clear the values and set the voltage.
            r = (r & !(0xF << 17)) | ((core as u32) << 17);

            // Write the modified value.
            write(ptr as *mut u32, r);
        }
    }
}


/// Available power profiles of the device.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PowerProfile {
    /// Low power profile (under 100 MHz).
    Low,

    /// Mid power profile (under 130 MHz).
    Mid,

    /// High power profile (under 150 MHz).
    High,
}

impl PowerProfile {
    /// Minimum frequency for LOW power profile [  0 MHz].
    const LPMINHZ: u32 = 0;

    /// Maximum frequency for LOW power profile [100 MHz].
    const LPMAXHZ: u32 = 100_000_000;

    /// Minimum frequency for MID power profile [100 MHz].
    const MPMINHZ: u32 = Self::LPMAXHZ + 1;

    /// Maximum frequency for MID power profile [130 MHz].
    const MPMAXHZ: u32 = 130_000_000;

    /// Returns the power profile that corresponds with the given frequency.
    pub const fn from(frequency: u32) -> Self {
        match frequency {
            Self::LPMINHZ..=Self::LPMAXHZ => PowerProfile::Low,
            Self::MPMINHZ..=Self::MPMAXHZ => PowerProfile::Mid,
            _ => PowerProfile::High,
        }
    }

    /// Returns the NMPA DCDC config values for the given profile.
    pub fn nmpa(&self) -> (u32, u32) {
        // Select the DCDC values address.
        let (a, b) = match self {
            PowerProfile::Low => {
                // Address of the DCDC Trim 0 and DCDC Trim 1 value.
                const TRIM0: u32 = 0x9FCE0;
                const TRIM1: u32 = 0x9FCE4;

                ( TRIM0, TRIM1 )
            },

            PowerProfile::Mid => {
                // Address of the DCDC Trim 0 and DCDC Trim 1 value.
                const TRIM0: u32 = 0x9FCE8;
                const TRIM1: u32 = 0x9FCEC;

                ( TRIM0, TRIM1 )
            },

            PowerProfile::High => {
                // Address of the DCDC Trim 0 and DCDC Trim 1 value.
                const TRIM0: u32 = 0x9FCD8;
                const TRIM1: u32 = 0x9FCDC;

                ( TRIM0, TRIM1 )
            },
        };

        // Read the values.
        let (mut dcdc0, dcdc1) = unsafe {(
            read(a as *const u32),
            read(b as *const u32),
        )};

        // Validate the DCDC 0 value.
        if (dcdc0 & 1) != 0 { dcdc0 = dcdc0 >> 1 }

        (dcdc0, dcdc1)
    }
}



/// Some NXP definitions of the device. Unknown usage.
/// Seems to be the quality of the process node.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ProcessNode {
    /// Slow.
    SSS,

    /// Average.
    NNN,

    /// Fast.
    FFF,
}

impl ProcessNode {
    /// Average Ring Oscillator frequency for the average process node.
    const NNNAVG: u32 = 19_300_000;

    /// Standard deviation of the NNNAVG value.
    const NNNSTD: u32 =    400_000;

    /// Maximum standard deviation limits for a process node.
    const NNNLIM: u32 =          6;

    /// Minimum Ring Oscillator frequency for the average process node.
    const NNNMIN: u32 = Self::NNNAVG - (Self::NNNLIM * Self::NNNSTD) + 1;

    /// Maximum Ring Oscillator frequency for the average process node.
    const NNNMAX: u32 = Self::NNNAVG + (Self::NNNLIM * Self::NNNSTD);

    /// Minimum Ring Oscillator frequency for the slow process node.
    const SSSMIN: u32 =          0;

    /// Maximum Ring Oscillator frequency for the slow process node.
    const SSSMAX: u32 = Self::NNNMIN - 1;

    /// Returns the core voltage for the given profile.
    pub fn voltage(&self, profile: PowerProfile) -> u32 {
        match self {
            // Slow process node.
            ProcessNode::SSS => match profile {
                PowerProfile::Low  => 1075,
                PowerProfile::Mid  => 1150,
                PowerProfile::High => 1200,
            },

            // Mid process node.
            ProcessNode::NNN => match profile {
                PowerProfile::Low  => 1000,
                PowerProfile::Mid  => 1100,
                PowerProfile::High => 1150,
            },

            // Fast process node.
            ProcessNode::FFF => match profile {
                PowerProfile::Low  => 1000,
                PowerProfile::Mid  => 1025,
                PowerProfile::High => 1050,
            },
        }
    }

    /// Gets the process node of the device.
    pub fn get() -> Self {
        // Read and validate both PVT RINGO values.
        let (ringo0, ringo1) = unsafe {(
            Self::validate( read(Self::RINGO0 as *const u32) ),
            Self::validate( read(Self::RINGO1 as *const u32) ),
        )};

        // Get the minimum RINGO.
        let ringo = core::cmp::min( ringo0, ringo1 );

        // Determine the process based on the value of the Ring Oscillator.
        match ringo {
            Self::SSSMIN..=Self::SSSMAX => ProcessNode::SSS,
            Self::NNNMIN..=Self::NNNMAX => ProcessNode::NNN,
            _ => ProcessNode::FFF,
        }
    }

    /// Internal function to validate a RINGO value.
    fn validate(ringo: u32) -> u32 {
        match ringo & 1 {
            0 => Self::NNNAVG,
            _ => ringo >> 1,
        }
    }
}
