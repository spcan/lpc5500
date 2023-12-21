



mod bus;
mod hardfault;
mod mem;
mod nmi;
mod systick;
mod usage;


pub use bus::{ BusFault, BusFaultReason, };
pub use usage::{ UsageFault, UsageFaultReason, };



use super::Vector;



/// Vector table of Core 0.
#[link_section = ".exceptions"]
pub(crate) static VTABLE0: [Vector; 14] = [
    // Non Maskable Interrupt exception.
    Vector::create( nmi::Handler ),

    // HardFault exception.
    Vector::create( hardfault::Handler ),

    // Memory Usage exception.
    Vector::create( mem::Handler ),

    // Bus Fault exception.
    Vector::create( bus::Handler ),

    // Usage Fault exception.
    Vector::create( usage::Handler ),

    // Secure Fault exception.
    Vector::create( Block ),

    // Reserved 8.
    Vector::reserved(),

    // Reserved 9.
    Vector::reserved(),

    // Reserved 10.
    Vector::reserved(),

    // Supervisor Call exception.
    Vector::create( Ignore ),

    // Debug Monitor exception.
    Vector::create( Ignore ),

    // Reserved 13.
    Vector::reserved(),

    // Pend Supervisor Call exception.
    Vector::create( Ignore ),

    // System Tick exception.
    Vector::create( Ignore ),
];



/// Reads the Configurable Fault Status Register.
#[inline(always)]
fn cfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED28 as *const u32) }
}


/// Default handler to block execution.
#[allow(non_snake_case)]
#[inline(never)]
unsafe extern "C" fn Block() {
    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}



/// Default handler to ignore exception.
#[allow(non_snake_case)]
#[inline(never)]
unsafe extern "C" fn Ignore() {
    return
}
