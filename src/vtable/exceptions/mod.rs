



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



use super::Vector;



/// Vector table of Core 0.
#[link_section = ".exceptions"]
#[used]
pub(crate) static VTABLE0: [Vector; 14] = [
    // Non Maskable Interrupt exception.
    Vector::handler( nmi::Handler ),

    // HardFault exception.
    Vector::handler( hardfault::Handler ),

    // Memory Usage exception.
    Vector::handler( mem::Handler ),

    // Bus Fault exception.
    Vector::handler( bus::Handler ),

    // Usage Fault exception.
    Vector::handler( usage::Handler ),

    // Secure Fault exception.
    Vector::handler( secure::Handler ),

    // Reserved 8.
    Vector::reserved(),

    // Reserved 9.
    Vector::reserved(),

    // Reserved 10.
    Vector::reserved(),

    // Supervisor Call exception.
    Vector::handler( svcall::Handler ),

    // Debug Monitor exception.
    Vector::handler( debug::Handler ),

    // Reserved 13.
    Vector::reserved(),

    // Pend Supervisor Call exception.
    Vector::handler( pendsv::Handler ),

    // System Tick exception.
    Vector::handler( systick::Handler ),
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
pub(super) unsafe extern "C" fn Block() {
    loop { core::arch::asm!("nop", options(nomem, nostack)) }
}



/// Default handler to ignore exception.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Ignore() {
    return
}
