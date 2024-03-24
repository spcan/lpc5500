//! DMA1 specific implementation.



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



/// DMA 1 descriptor list.
#[link_section = ".bss.LPC5500.dma.DMA1DESCRIPTORS"]
pub(super) static mut DMA1DESCRIPTORS: DescriptorList<'static, 10> = DescriptorList::empty();



pub struct DMA1;

impl Control for DMA1 {
    const REG: u32 = 2;
    const OFF: u8 = 1;
}

impl Enable  for DMA1 {}
impl Unreset for DMA1 {}

unsafe impl Disable for DMA1 {}
unsafe impl Reset   for DMA1 {}
