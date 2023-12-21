//! Available power profiles of the device.



use core::ptr::read_volatile as read;



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature="defmt", derive(defmt::Format))]
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

    /// Base address of the NMPA space.
    const NMPA: u32 = 0x9FC00;

    /// Offset of the DCDC 0 value for low power.
    const LPTRIM0: u32 = Self::NMPA + 0xE0;

    /// Offset of the DCDC 1 value for low power.
    const LPTRIM1: u32 = Self::NMPA + 0xE4;

    /// Offset of the DCDC 0 value for medium power.
    const MPTRIM0: u32 = Self::NMPA + 0xE8;

    /// Offset of the DCDC 1 value for medium power.
    const MPTRIM1: u32 = Self::NMPA + 0xEC;

    /// Offset of the DCDC 0 value for high power.
    const HPTRIM0: u32 = Self::NMPA + 0xD8;

    /// Offset of the DCDC 1 value for high power.
    const HPTRIM1: u32 = Self::NMPA + 0xDC;

    /// Returns the power profile that corresponds with the given frequency.
    pub(super) const fn from(frequency: u32) -> Self {
        match frequency {
            Self::LPMINHZ..=Self::LPMAXHZ => PowerProfile::Low,
            Self::MPMINHZ..=Self::MPMAXHZ => PowerProfile::Mid,
            _ => PowerProfile::High,
        }
    }

    /// Returns the NMPA DCDC config values for the given profile.
    pub(super) fn dcdc(&self) -> (u32, u32) {
        // Select the DCDC values address.
        let (a, b) = match self {
            PowerProfile::Low  => (Self::LPTRIM0, Self::LPTRIM1),

            PowerProfile::Mid  => (Self::MPTRIM0, Self::MPTRIM1),

            PowerProfile::High => (Self::HPTRIM0, Self::HPTRIM1),
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
