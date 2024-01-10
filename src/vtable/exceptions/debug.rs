//! Debug exception.



/// Function handler of the Debug exception.
#[allow(non_snake_case)]
#[inline(never)]
pub(super) unsafe extern "C" fn Handler() {
    //loop { core::arch::asm!("nop", options(nomem, nostack)) }
    return;
}
