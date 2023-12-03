



mod bus;
mod mem;
mod usage;


pub use bus::{ BusFault, BusFaultReason, };
pub use usage::{ UsageFault, UsageFaultReason, };



use super::Vector;



/// Vector table of Core 0.
pub(crate) static VTABLE0: [Vector; 16] = [
    // Initial Stack Pointer vector.
    Vector::reserved(),

    // Reset exception.
    Vector::reserved(),

    // Non_Maskable interrupt exception.
    Vector::reserved(),

    // HardFault exception.
    Vector::reserved(),

    // Memory Usage exception.
    Vector::reserved(),

    // Bus Fault exception.
    Vector::reserved(),

    // Usage Fault exception.
    Vector::reserved(),

    // Secure Fault exception.
    Vector::reserved(),

    // Reserved 8.
    Vector::reserved(),

    // Reserved 9.
    Vector::reserved(),

    // Reserved 10.
    Vector::reserved(),

    // Supervisor Call exception.
    Vector::reserved(),

    // Debug Monitor exception.
    Vector::reserved(),

    // Reserved 13.
    Vector::reserved(),

    // Pend Supervisor Call exception.
    Vector::reserved(),

    // System Tick exception.
    Vector::reserved(),
];



/// Reads the Configurable Fault Status Register.
#[inline(always)]
fn cfsr() -> u32 {
    unsafe { core::ptr::read_volatile(0xE000ED28 as *const u32) }
}