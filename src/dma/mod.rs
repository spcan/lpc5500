//! Direct Memory Access (DMA) control module.



mod channel;
mod descriptor;
mod transfer;



pub use channel::Channel;



/// DMA 0 instance.
pub struct DMA0 {
    /// Channel 0 of DMA 0.
    pub ch0 : Channel<0,  0>,

    /// Channel 1 of DMA 0.
    pub ch1 : Channel<0,  1>,

    /// Channel 2 of DMA 0.
    pub ch2 : Channel<0,  2>,

    /// Channel 3 of DMA 0.
    pub ch3 : Channel<0,  3>,

    /// Channel 4 of DMA 0.
    pub ch4 : Channel<0,  4>,

    /// Channel 5 of DMA 0.
    pub ch5 : Channel<0,  5>,

    /// Channel 6 of DMA 0.
    pub ch6 : Channel<0,  6>,

    /// Channel 7 of DMA 0.
    pub ch7 : Channel<0,  7>,

    /// Channel 8 of DMA 0.
    pub ch8 : Channel<0,  8>,

    /// Channel 9 of DMA 0.
    pub ch9 : Channel<0,  9>,

    /// Channel 10 of DMA 0.
    pub ch10: Channel<0, 10>,

    /// Channel 11 of DMA 0.
    pub ch11: Channel<0, 11>,

    /// Channel 12 of DMA 0.
    pub ch12: Channel<0, 12>,

    /// Channel 13 of DMA 0.
    pub ch13: Channel<0, 13>,

    /// Channel 14 of DMA 0.
    pub ch14: Channel<0, 14>,

    /// Channel 15 of DMA 0.
    pub ch15: Channel<0, 15>,

    /// Channel 16 of DMA 0.
    pub ch16: Channel<0, 16>,

    /// Channel 17 of DMA 0.
    pub ch17: Channel<0, 17>,

    /// Channel 18 of DMA 0.
    pub ch18: Channel<0, 18>,

    /// Channel 19 of DMA 0.
    pub ch19: Channel<0, 19>,

    /// Channel 20 of DMA 0.
    pub ch20: Channel<0, 20>,

    /// Channel 21 of DMA 0.
    pub ch21: Channel<0, 21>,

    /// Channel 22 of DMA 0.
    pub ch22: Channel<0, 22>,
}

impl DMA0 {
    /// Creates the initial singleton of the DMA 0.
    pub(super) fn create() -> Self {
        // Initialize the SRAM descriptors address.
        unsafe {
            core::ptr::write_volatile(
                0x40082008 as *mut u32,
                core::ptr::addr_of!( descriptor::DMA0DESCRIPTORS ) as u32,
            )
        }

        Self {
            ch0 : Channel::<0,  0>::create(),
            ch1 : Channel::<0,  1>::create(),
            ch2 : Channel::<0,  2>::create(),
            ch3 : Channel::<0,  3>::create(),
            ch4 : Channel::<0,  4>::create(),
            ch5 : Channel::<0,  5>::create(),
            ch6 : Channel::<0,  6>::create(),
            ch7 : Channel::<0,  7>::create(),
            ch8 : Channel::<0,  8>::create(),
            ch9 : Channel::<0,  9>::create(),
            ch10: Channel::<0, 10>::create(),
            ch11: Channel::<0, 11>::create(),
            ch12: Channel::<0, 12>::create(),
            ch13: Channel::<0, 13>::create(),
            ch14: Channel::<0, 14>::create(),
            ch15: Channel::<0, 15>::create(),
            ch16: Channel::<0, 16>::create(),
            ch17: Channel::<0, 17>::create(),
            ch18: Channel::<0, 18>::create(),
            ch19: Channel::<0, 19>::create(),
            ch20: Channel::<0, 20>::create(),
            ch21: Channel::<0, 21>::create(),
            ch22: Channel::<0, 22>::create(),
        }
    }
}



/// DMA 1 instance.
pub struct DMA1 {
    /// Channel 0 of DMA 1.
    pub ch0 : Channel<1,  0>,

    /// Channel 1 of DMA 1.
    pub ch1 : Channel<1,  1>,

    /// Channel 2 of DMA 1.
    pub ch2 : Channel<1,  2>,

    /// Channel 3 of DMA 1.
    pub ch3 : Channel<1,  3>,

    /// Channel 4 of DMA 1.
    pub ch4 : Channel<1,  4>,

    /// Channel 5 of DMA 1.
    pub ch5 : Channel<1,  5>,

    /// Channel 6 of DMA 1.
    pub ch6 : Channel<1,  6>,

    /// Channel 7 of DMA 1.
    pub ch7 : Channel<1,  7>,

    /// Channel 8 of DMA 1.
    pub ch8 : Channel<1,  8>,

    /// Channel 9 of DMA 1.
    pub ch9 : Channel<1,  9>,
}

impl DMA1 {
    /// Creates the initial singleton of the DMA 1.
    pub(super) fn create() -> Self {
        // Initialize the SRAM descriptors address.
        unsafe {
            core::ptr::write_volatile(
                0x400A7008 as *mut u32,
                core::ptr::addr_of!( descriptor::DMA1DESCRIPTORS ) as u32,
            )
        }

        Self {
            ch0: Channel::<1,  0>::create(),
            ch1: Channel::<1,  1>::create(),
            ch2: Channel::<1,  2>::create(),
            ch3: Channel::<1,  3>::create(),
            ch4: Channel::<1,  4>::create(),
            ch5: Channel::<1,  5>::create(),
            ch6: Channel::<1,  6>::create(),
            ch7: Channel::<1,  7>::create(),
            ch8: Channel::<1,  8>::create(),
            ch9: Channel::<1,  9>::create(),
        }
    }
}
