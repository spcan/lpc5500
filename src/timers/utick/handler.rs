//! Micro tick interrupt module.



#![allow(non_snake_case)]



/// Global flag to indicate the micro tick timer has triggered.
#[link_section = ".bss.UTICK"]
pub static mut TRIGGERED: u32 = 0;



/// Micro tick interrupt handler.
#[inline(never)]
pub extern "C" fn UTICK() {
    unsafe {
        // Mark the timer trigger.
        TRIGGERED = 7;

        // Unpend the interrupt in the peripheral.
        core::ptr::write_volatile( 0x5000E004 as *mut u32, 1 );

        // Unpend the interrupt in the NVIC.
        core::ptr::write_volatile(0xE000E280 as *mut u32, 1 << 8);
    }
}
