//! Lists of the possible voltages in the power domains of the device.







/// Defines the possible values of the Core domain voltage levels (in mV).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CoreVoltage {
    /// 0.950 mV.
    V0950 =  0,

    /// 0.975 mV.
    V0975 =  1,

    /// 1.000 mV.
    V1000 =  2,

    /// 1.025 mV.
    V1025 =  3,

    /// 1.050 mV.
    V1050 =  4,

    /// 1.075 mV.
    V1075 =  5,

    /// 1.100 mV.
    V1100 =  6,

    /// 1.125 mV.
    V1125 =  7,

    /// 1.150 mV.
    V1150 =  8,

    /// 1.175 mV.
    V1175 =  9,

    /// 1.200 mV.
    V1200 = 10,
}

impl CoreVoltage {
    /// Returns the valid core voltage that provides at least the given voltage.
    pub fn from(voltage: u32) -> Self {
        match voltage {
               0..= 950 => Self::V0950,
             951..= 975 => Self::V0975,
             976..=1000 => Self::V1000,
            1001..=1025 => Self::V1025,
            1026..=1050 => Self::V1050,
            1051..=1075 => Self::V1075,
            1076..=1100 => Self::V1100,
            1101..=1125 => Self::V1125,
            1126..=1150 => Self::V1150,
            1151..=1175 => Self::V1175,
            _           => Self::V1200,

        }
    }
}



/// Defines the possible values of the Always-On domain voltage levels (in mV).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AOVoltage {
    /// 0.700 V.
    V0700 =  1,

    /// 0.725 V.
    V0725 =  2,

    /// 0.750 V.
    V0750 =  3,

    /// 0.775 V.
    V0775 =  4,

    /// 0.800 V.
    V0800 =  5,

    /// 0.825 V.
    V0825 =  6,

    /// 0.850 V.
    V0850 =  7,

    /// 0.875 V.
    V0875 =  8,

    /// 0.900 V.
    V0900 =  9,

    /// 0.960 V.
    V0960 = 10,

    /// 0.970 V.
    V0970 = 11,

    /// 0.980 V.
    V0980 = 12,

    /// 0.990 V.
    V0990 = 13,

    /// 1.000 V.
    V1000 = 14,

    /// 1.010 V.
    V1010 = 15,

    /// 1.020 V.
    V1020 = 16,

    /// 1.030 V.
    V1030 = 17,

    /// 1.040 V.
    V1040 = 18,

    /// 1.050 V.
    V1050 = 19,

    /// 1.060 V.
    V1060 = 20,

    /// 1.070 V.
    V1070 = 21,

    /// 1.080 V.
    V1080 = 22,

    /// 1.090 V.
    V1090 = 23,

    /// 1.100 V.
    V1100 = 24,

    /// 1.110 V.
    V1110 = 25,

    /// 1.120 V.
    V1120 = 26,

    /// 1.130 V.
    V1130 = 27,

    /// 1.140 V.
    V1140 = 28,

    /// 1.150 V.
    V1150 = 29,

    /// 1.160 V.
    V1160 = 30,

    /// 1.220 V.
    V1220 = 31,
}

impl AOVoltage {
    /// Returns the AO voltage and AO Boost voltage for a given core voltage.
    pub fn from(core: CoreVoltage) -> (Self, Self) {
        match core {
            // Set AO domain to 960 mV (boost to 1010 mV).
            CoreVoltage::V0950 => (
                AOVoltage::V0960,
                AOVoltage::V1010,
            ),

            // Set AO domain to 980 mV (boost to 1030 mV).
            CoreVoltage::V0975 => (
                AOVoltage::V0980,
                AOVoltage::V1030,
            ),

            // Set AO domain to 1000 mV (boost to 1050 mV).
            CoreVoltage::V1000 => (
                AOVoltage::V1000,
                AOVoltage::V1050,
            ),

            // Set AO domain to 1030 mV (boost to 1080 mV).
            CoreVoltage::V1025 => (
                AOVoltage::V1030,
                AOVoltage::V1080,
            ),

            // Set AO domain to 1060 mV (boost to 1110 mV).
            CoreVoltage::V1050 => (
                AOVoltage::V1060,
                AOVoltage::V1110,
            ),

            // Set AO domain to 1080 mV (boost to 1130 mV).
            CoreVoltage::V1075 => (
                AOVoltage::V1080,
                AOVoltage::V1130,
            ),

            // Set AO domain to 1100 mV (boost to 1150 mV).
            CoreVoltage::V1100 => (
                AOVoltage::V1100,
                AOVoltage::V1150,
            ),

            // Set AO domain to 1130 mV (boost to 1160 mV).
            CoreVoltage::V1125 => (
                AOVoltage::V1130,
                AOVoltage::V1160,
            ),

            // Set AO domain to 1160 mV (boost to 1220 mV).
            CoreVoltage::V1150 => (
                AOVoltage::V1160,
                AOVoltage::V1220,
            ),

            // Set AO domain to 1160 mV (boost to 1220 mV).
            CoreVoltage::V1175 => (
                AOVoltage::V1160,
                AOVoltage::V1220,
            ),

            // Set AO domain to 1160 mV (boost to 1220 mV).
            CoreVoltage::V1200 => (
                AOVoltage::V1160,
                AOVoltage::V1220,
            ),
        }
    }
}
