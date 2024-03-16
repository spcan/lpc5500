//! Micro tick timer module.



pub mod handler;



/// Simple and ultra low-power timer.
/// Can run and wake the device from deep-sleep mode.
pub struct MicroTick;

impl MicroTick {
    /// Creates an instance of the Micro tick timer.
    pub(crate) fn create() -> Self {
        Self
    }

    /// Initializes the micro tick timer.
    pub fn init(&mut self) {
        use crate::vtable::{ Vector, VectorTable };

        // Initialize the peripheral.
        crate::system::SystemControl::init::<Self>();

        unsafe {
            // Get the current vector table.
            let vtable = VectorTable::<60>::current();

            // Register the handler.
            vtable.interrupt(8, Vector::handler(handler::UTICK), None);
        }
    }

    /// Returns `true` if the timer is counting.
    #[inline]
    pub fn active(&self) -> bool {
        (unsafe { core::ptr::read_volatile(0x5000E004 as *const u32) } & 0b10) != 0
    }

    /// Returns `true` if the timer's interrupr is pending.
    #[inline]
    pub fn pending(&self) -> bool {
        (unsafe { core::ptr::read_volatile(0x5000E004 as *const u32) } & 0b01) != 0
    }

    /// Configures the delay of the micro tick timer and starts the timer.
    /// If `repeat` is set, the timer will restart with the same delay after triggering.
    #[inline]
    pub fn start(&mut self, delay: u32, repeat: bool) {
        // Create the register value.
        let src = (delay & 0x7FFFFFFF) | (if repeat { 1 } else { 0 } << 31);

        // Write the register value.
        unsafe { core::ptr::write_volatile( 0x5000E000 as *mut u32, src ) }
    }

    /// Stops the micro tick timer.
    #[inline]
    pub fn stop(&mut self) {
        unsafe { core::ptr::write_volatile( 0x5000E000 as *mut u32, 0 ) }
    }
}

impl crate::system::Control for MicroTick {
    const REG: u32 = 1;
    const OFF: u8 = 10;
}

impl crate::system::enable::Enable  for MicroTick {}
impl crate::system::reset::Unreset for MicroTick {}

unsafe impl crate::system::enable::Disable for MicroTick {}
unsafe impl crate::system::reset::Reset   for MicroTick {}
