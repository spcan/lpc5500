//! PowerQuad Engine handler.



mod container;



pub(crate) use container::Container;



/// Global container for the PowerQuad handler.
#[link_section = ".bss.POWERQUAD"]
#[used]
pub(crate) static mut CONTAINER: Container = Container::empty();



/// PowerQuad engine Interrupt Handler.
#[inline(never)]
pub(crate) extern "C" fn handler() {
    use super::Error;

    // Ordered list of error bits.
    const ERRORS: [Error; 5] = [Error::Overflow, Error::Nan, Error::FixedOverflow, Error::Underflow, Error::Bus];

    // Read the Error Status.
    let status = unsafe { core::ptr::read_volatile((0x500A6000 + 0x18C) as *const u32) };

    // Clear the Interrupt Pending, Interrupt Status and Error Status.
    unsafe { core::ptr::write_volatile((0xE000E100u32 + 0x184) as *mut u32, 1 << 25) };
    unsafe { core::ptr::write_volatile((0x500A6000u32 + 0x198) as *mut u32, 1      ) };
    unsafe { core::ptr::write_volatile((0x500A6000u32 + 0x18C) as *mut u32, status ) };

    // Mark the operation as completed (successful or not) and store the errors.
    unsafe {
        CONTAINER.end = true;
        CONTAINER.error = status;
    }
}
