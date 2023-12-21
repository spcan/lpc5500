//! Direct Memory Access (DMA) module.



pub mod channel;
pub mod common;
pub mod descriptor;
pub mod endpoint;
pub mod transfer;

mod error;
mod dma0;
mod dma1;



pub use error::DMAError;

use crate::system::SystemControl;
use common::Width;



pub(crate) fn init() {
    use dma0::DMA0;
    use dma1::DMA1;

    // Reset both DMAs.
    SystemControl::reset::<DMA0>();
    SystemControl::reset::<DMA1>();

    // Enable clocks to both DMAs.
    SystemControl::enable::<DMA0>();
    SystemControl::enable::<DMA1>();

    // Unreset both DMAs.
    SystemControl::unreset::<DMA0>();
    SystemControl::unreset::<DMA1>();

    // Enable both DMA masters.
    unsafe {
        use core::ptr::write_volatile as write;

        write( 0x40082000 as *mut u32, 1 );
        write( 0x400A7000 as *mut u32, 1 );
    }
}



/// Common trait for all the data that can be transfered by the DMA.
pub trait DMAData: Sized {
    /// The width of the data.
    const WIDTH: Width;
}

impl DMAData for f32 {
    const WIDTH: Width = Width::Word;
}

impl DMAData for u32 {
    const WIDTH: Width = Width::Word;
}

impl DMAData for i32 {
    const WIDTH: Width = Width::Word;
}

impl DMAData for u16 {
    const WIDTH: Width = Width::Half;
}

impl DMAData for i16 {
    const WIDTH: Width = Width::Half;
}

impl DMAData for u8 {
    const WIDTH: Width = Width::Byte;
}

impl DMAData for i8 {
    const WIDTH: Width = Width::Byte;
}
