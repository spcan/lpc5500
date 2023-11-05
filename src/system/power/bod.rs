//! Brown Out Detector module.








/// Hysteresis in the BOD settings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Hysteresis {
    /// 25 mV hysteresis.
    Level25mV = 0b00,

    /// 50 mV hysteresis.
    Level50mV = 0b01,

    /// 75 mV hysteresis.
    Level75mV = 0b10,

    /// 100 mV hysteresis.
    Level100mV = 0b11,
}
