//! Runtime vector tables.



use super::*;



#[link_section = ".vtable.CORE0"]
#[used]
pub static mut CORE0VTABLE: VectorTable<60> = VectorTable::create();



#[link_section = ".vtable.CORE1"]
#[used]
pub static mut CORE1VTABLE: VectorTable<60> = VectorTable::create();
