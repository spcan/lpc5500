//! Internal Free Running Oscillators modules.



pub(crate) mod fro1m;
pub(crate) mod fro12m;
pub(crate) mod fro32k;
pub(crate) mod fro48m;
pub(crate) mod fro96m;



pub(super) use self::{
    fro1m::FRO1MHz,
    fro12m::FRO12MHz,
    fro32k::FRO32KHz,
    fro48m::FRO48MHz,
    fro96m::FRO96MHz,
};



pub(super) struct FROControl;

impl FROControl{
    /// Enables the given FRO clock signal.
    pub(self) fn enable<S: FROEnable>() {
        unsafe {
            // Address of the CTRL register [ANACTRL->FRO192M_CTRL @ 0x50013000 + 0x10].
            const ADDRESS: usize = 0x50013000 + 0x10;

            // Read the CTRL register .
            let ctrl = core::ptr::read_volatile( ADDRESS as *const u32 );

            // Write the modified CTRL register.
            core::ptr::write_volatile(ADDRESS as *mut u32, ctrl | (1 << S::OFFSET));
        }
    }

    /// Disables the given FRO clock signal.
    pub(self) fn disable<S: FRODisable>() {
        unsafe {
            // Address of the CTRL register [ANACTRL->FRO192M_CTRL @ 0x50013000 + 0x10].
            const ADDRESS: usize = 0x50013000 + 0x10;

            // Read the CTRL register .
            let ctrl = core::ptr::read_volatile( ADDRESS as *const u32 );

            // Write the modified CTRL register.
            core::ptr::write_volatile(ADDRESS as *mut u32, ctrl & !(1 << S::OFFSET));
        }
    }
}



/// Common trait for all FRO signals that can be enabled.
pub(self) trait FROEnable {
    /// Indicates the in-register offset of the control bit.
    const OFFSET: u8;
}



/// Common trait for all FRO signals that can be disabled.
pub(self) trait FRODisable: FROEnable {}
