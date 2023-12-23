



pub(self) mod bus;
pub(self) mod debug;
pub(self) mod hardfault;
pub(self) mod mem;
pub(self) mod nmi;
pub(self) mod pendsv;
pub(self) mod secure;
pub(self) mod svcall;
pub(self) mod systick;
pub(self) mod usage;


pub use bus::{ BusFault, BusFaultReason, };
pub use usage::{ UsageFault, UsageFaultReason, };



use super::Vector;



/// Vector table of Core 0.
#[link_section = ".exceptions"]
#[used]
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
    Vector::create( secure::Handler ),

    // Reserved 8.
    Vector::reserved(),

    // Reserved 9.
    Vector::reserved(),

    // Reserved 10.
    Vector::reserved(),

    // Supervisor Call exception.
    Vector::create( svcall::Handler ),

    // Debug Monitor exception.
    Vector::create( debug::Handler ),

    // Reserved 13.
    Vector::reserved(),

    // Pend Supervisor Call exception.
    Vector::create( pendsv::Handler ),

    // System Tick exception.
    Vector::create( systick::Handler ),
];



/// Reads the Configurable Fault Status Register.
#[inline(always)]
fn cfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED28 as *const u32) }
}

/// Reads the Secure Fault Status Register.
#[inline(always)]
fn sfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000EDE4 as *const u32) }
}

/// Reads the Hard Fault Status Register.
#[inline(always)]
fn hfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED2C as *const u32) }
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
