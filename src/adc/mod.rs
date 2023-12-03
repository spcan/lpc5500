//! Analog to Digital Conversion peripheral.



use core::ptr::{
    read_volatile  as read ,
    write_volatile as write,
};

use crate::system::{
    Control, SystemControl,

    enable::{
        Disable, Enable,
    },
    reset::{
        Reset, Unreset,
    },
};



pub struct Adc;

impl Adc {
    // SYSCON peripheral base address.
    const SYSCON: u32 = 0x50000000;

    // ADCCLKSEL register address.
    const ADCCLKSEL: u32 = Self::SYSCON + 0x2A4;

    // ADCCLKDIV register address.
    const ADCCLKDIV: u32 = Self::SYSCON + 0x394;

    // PMC peripheral base address.
    const PMC: u32 = 0x40020000;

    // PDRUNCLR register address.
    const PDRUNCLR: u32 = Self::PMC + 0xC8;

    // ADC peripheral base address.
    const ADC: u32 = 0x500A0000;

    // ADC CTRL register offset.
    const CTRL: u32 = Self::ADC + 0x10;

    // ADC CFG register address.
    const CFG: u32 = Self::ADC + 0x20;

    /// Initializes the ADC.
    pub fn init(&mut self/*, config: Config*/) {
        // Reset the peripheral.
        SystemControl::reset<Self>();

        // Enable the clock to the peripheral.
        SystemControl::enable<Self>();

        // Unreset the peripheral.
        SystemControl::unreset<Self>();

        // Enable the clock to the peripheral.
        SystemControl::enable<Self>();

        unsafe {
            // Reset the ADC clock div.
            write( Self::ADCCLKDIV as *mut u32, 1 << 30 );

            // Set the ADC clock divider to 4.
            write( Self::ADCCLKDIV as *mut u32, 4 );

            // Set the ADC clock source to FRO 96 MHz.
            write( Self::ADCCLKSEL as *mut u32, 2 );

            // Enable the ADC, Aux Bias and Temperature Sensor in the PMC.
            write( Self::PDRUNCLR as *mut u32, (1 << 19) | (1 << 15) |(1 << 14) );

            // Reset the ADC, FIFO0 and FIFO1.
            let ctrl = read( Self::CTRL as *const u32 );
            write( Self::CTRL as *mut u32, ctrl | ((1 << 9) | (1 << 8) | (1 << 1)) );

            // Unreset the ADC, FIFO0 and FIFO1.
            write(Self::CTRL as *mut u32, ctrl & !((1 << 9) | (1 << 8) | (1 << 1)) );

            // Disable the ADC and ADC power.
            let ctrl = read( Self::CTRL as *const u32 );
            write( Self::CTRL as *mut u32, ctrl & !1 );

            let cfg = read( Self::CFG as *const u32 );
            write( Self::CTRL as *mut u32, cfg & !(1 << 28) );
        }
    }
}

impl Control for Adc {
    const REG: u32 = 0;
    const OFF: u8 = 27;
}

impl Unreset for Adc {}
impl Enable  for Adc {}

unsafe impl Reset   for Adc {}
unsafe impl Disable for Adc {}
