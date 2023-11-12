//! Global peripheral singleton.



#![allow(non_snake_case)]



use crate::{
    gpio::Pins,
    //system::clocks::main::MainClock,
    system::user::UserSystemControl,
};



pub struct Peripherals {
    /// Main clock control.
    //pub mainclk: MainClock,

    /// Collection of all pins in the device.
    pub pins: Pins,

    /// User facing System Control interface.
    pub user: UserSystemControl,
}
