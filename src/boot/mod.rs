//! NXP LPC5500 series boot sequence.



pub mod vectortable;



/// Default reset of the library.
/// This function will always handle the behaviour of the chip after a reset.
pub fn DefaultReset() -> ! {
    // Load the `.data` section in memory and reset the `.bss` memory.

    // SAFETY : TODO : Zero out the `.uninit` section to avoid leaking secrets.

    // Reinitialize the peripherals.

    loop {}
}
