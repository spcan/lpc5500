//! Implementations of the internal oscillators.



use crate::system::{
    clocks::ClockEnable,
    pdrun::PowerOn,
};



/// Internal FRO running at 1 MHz.
pub struct FRO1MHz;

impl ClockEnable for FRO1MHz {
    fn enable(&mut self) {
        // Enable the clock in the PD RUN register.
        self.poweron();
    }
}

impl PowerOn for FRO1MHz {
    const OFFSET: u8 = 4;
}



/// Internal FRO running at 12 MHz.
pub struct FRO12MHz;

impl super::traits::FROEnable for FRO12MHz {
    const OFFSET: u8 = 14;
}

impl super::traits::FRODisable for FRO12MHz {}



/// Internal FRO running at 48 MHz.
pub struct FRO48MHz;

impl super::traits::FROEnable for FRO48MHz {
    const OFFSET: u8 = 15;
}



/// Internal FRO running at 96 MHz.
pub struct FRO96MHz;

impl super::traits::FROEnable for FRO96MHz {
    const OFFSET: u8 = 30;
}

impl super::traits::FRODisable for FRO96MHz {}



/// Internal FRO running at 32 kHz.
pub struct FRO32KHz;

impl ClockEnable for FRO32KHz {
    fn enable(&mut self) {
        // Enable the clock in the PD RUN register.
        self.poweron();
    }
}

impl PowerOn for FRO32KHz {
    const OFFSET: u8 = 6;
}
