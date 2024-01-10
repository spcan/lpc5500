//! Core 1 (Slave) vector table implementation.



use super::VectorTable;



#[link_section = ".interrupts.CORE1"]
#[used]
pub(crate) static mut C1INTERRUPTS: VectorTable<60> = VectorTable::create();
