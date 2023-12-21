//! This an NXP defined characteristic that categorizes the devices by the
//! quality of its process node. Better nodes run faster with less voltage.



use core::ptr::read_volatile as read;

use super::PowerProfile;



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature="defmt", derive(defmt::Format))]
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

    /// Base NMPA address
    const NMPA: u32 = 0x9FC00;

    /// Address of the PVT Monitor 0 RINGO.
    const RINGO0: u32 = Self::NMPA + 0x130;

    /// Address of the PVT Monitor 1 RINGO.
    const RINGO1: u32 = Self::NMPA + 0x140;

    /// Returns the core voltage for the given profile.
    pub(super) fn voltage(&self, profile: PowerProfile) -> u32 {
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
    pub(super) fn get() -> Self {
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
