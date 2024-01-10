//! Core 0 vector table.



use super::*;



#[link_section = ".vtable.CORE0"]
#[used]
pub static mut CORE0VTABLE: VectorTable<60> = VectorTable::create();
