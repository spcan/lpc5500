//! DMA0 specific implementation.



use crate::system::{
    Control,
    enable::{
        Disable, Enable,
    },
    reset::{
        Reset, Unreset,
    },
};

use super::descriptor::DescriptorList;



/// DMA 0 descriptor list.
#[link_section = ".bss.LPC5500.dma.DMA0DESCRIPTORS"]
pub(super) static mut DMA0DESCRIPTORS: DescriptorList<'static, 23> = DescriptorList::empty();



pub struct DMA0;

impl Control for DMA0 {
    const REG: u32 = 0;
    const OFF: u8 = 20;
}

impl Enable  for DMA0 {}
impl Unreset for DMA0 {}

unsafe impl Disable for DMA0 {}
unsafe impl Reset   for DMA0 {}
