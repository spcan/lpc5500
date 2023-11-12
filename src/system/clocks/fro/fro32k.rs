//! Internal FRO at 32 kHz.



use crate::system::pdrun::PowerOn;



pub struct FRO32kHz;

impl FRO32kHz {
    /// Initializes the 32 kHz FRO.
    pub(super) fn init() {
        Self.poweron();
    }
}

impl PowerOn for FRO32kHz {
    const OFFSET: u8 = 6;
}
