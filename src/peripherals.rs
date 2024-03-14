//! Global peripheral singleton.



#![allow(non_snake_case)]



use crate::{
    gpio::Pins,

    powerquad::PowerQuad,

    system::user::UserSystemControl,
    
    timers::{
        ostimer::OSTime,
        utick::MicroTick,
    },
};



pub struct Peripherals {
    /// Collection of all pins in the device.
    pub pins: Pins,

    /// PowerQuad coprocessor interface.
    pub powerquad: PowerQuad,

    /// User facing System Control interface.
    pub user: UserSystemControl,

    /// Global OS time access.
    pub ostime: OSTime,

    /// Micro second timer.
    pub utick: MicroTick,
}
