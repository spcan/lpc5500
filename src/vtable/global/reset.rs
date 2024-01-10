//! The default reset vector table.



use super::*;



extern "C" {
    // Linker symbol for the end of the main stack.
    static __estack0: u32;
}



#[link_section = ".vtable.RESET"]
#[used]
pub static RESETVTABLE: VectorTable<0> = VectorTable::preloaded([
    // Default SP.
    Vector::raw( 0x20000000 + (16 * 1024) - 4 ),

    // Default Reset handler.
    Vector::handler( crate::Reset ),

    // NMI exception.
    Vector::handler( Block ),

    // HardFault exception.
    Vector::handler( Block ),

    // Memory Usage exception.
    Vector::handler( Block ),

    // Bus Fault exception.
    Vector::handler( Block ),

    // Usage Fault exception.
    Vector::handler( Block ),

    // Secure Fault exception.
    Vector::handler( Block ),

    // Reserved 8.
    Vector::reserved(),

    // Reserved 9.
    Vector::reserved(),

    // Reserved 10.
    Vector::reserved(),

    // Supervisor Call exception.
    Vector::handler( Ignore ),

    // Debug Monitor exception.
    Vector::handler( Ignore ),

    // Reserved 13.
    Vector::reserved(),

    // Pend Supervisor Call exception.
    Vector::handler( Ignore ),

    // System Tick exception.
    Vector::handler( Ignore ),
]);
