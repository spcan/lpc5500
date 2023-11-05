

#![no_std]
#![no_main]

extern crate lpc5500;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}