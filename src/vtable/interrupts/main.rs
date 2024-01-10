//! Core 0 (Main) vector table implementation.



use super::VectorTable;



#[link_section = ".interrupts.CORE0"]
#[used]
pub(crate) static mut C0INTERRUPTS: VectorTable<60> = VectorTable::create();
