//! OS timer module.



//pub mod handler;
mod reader;


pub use reader::OSTime;



/// Long running (4 million years @ 32 kHz) OS event timer.
/// Can trigger an alarm on timer match and register event time.
/// Can be a wakeup source in deep power down.
pub struct OSTimer;

impl OSTimer {
    /// Creates an instance of the OS timer.
    pub(crate) fn create() -> Self {
        Self
    }

    /// Initializes the OS timer.
    pub fn init(self) -> OSTime {
        use core::ptr::{
            read_volatile  as read,
            write_volatile as write,
        };

        // Enable the OS timer 32 kHz and reset the timer clock in PMC - OSTIMER.
        let mut src = unsafe { read(0x4002009C as *const u32) };
        src |= (1 << 1) | 1;
        src &= !(1 << 3);
        unsafe { write(0x4002009C as *mut u32, src) };

        // Unreset the OS timer.
        src &= !1;
        unsafe { write(0x4002009C as *mut u32, src) };

        // Enable the OS timer.
        crate::system::SystemControl::init::<Self>();

        OSTime
    }
}

impl crate::system::Control for OSTimer {
    const REG: u32 = 1;
    const OFF: u8 = 1;
}

impl crate::system::enable::Enable  for OSTimer {}
impl crate::system::reset::Unreset for OSTimer {}

unsafe impl crate::system::enable::Disable for OSTimer {}
unsafe impl crate::system::reset::Reset   for OSTimer {}
