//! Internal traits of FRO clock sources.



/// Common trait for all FRO signals that can be enabled.
pub(super) trait FROEnable {
    /// Indicates the in-register offset of the control bit.
    const OFFSET: u8;
}



/// Common trait for all FRO signals that can be disabled.
pub(super) trait FRODisable: FROEnable {}
