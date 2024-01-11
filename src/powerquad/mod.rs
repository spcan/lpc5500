//! PowerQuad Coprocessor module.



pub mod coprocessor;
pub mod engine;

mod sleep;



use core::sync::atomic::{
    AtomicU32, Ordering,
};

use coprocessor::Coprocessor;

use crate::system::{
    Control, SystemControl,
    enable::{
        Disable, Enable,
    },
    reset::{
        Reset, Unreset,
    },
};

use engine::Engine;



#[link_section = ".data.LPC5500.powerquad.POWEROFF"]
pub(self) static POWER: AtomicU32 = AtomicU32::new( 0 );



pub struct PowerQuad;

impl PowerQuad {
    /// Internal function to create an instance.
    pub(crate) fn create() -> Self {
        Self
    }

    /// Initializes the PowerQuad.
    /// Returns all the interfaces to the PowerQuad.
    /// <div class="warning">Whichever core initializes the PowerQuad becomes
    /// responsible for the PowerQuad interrupt handling.</div>
    pub fn init(self) -> (Coprocessor<0>, Coprocessor<1>, Engine) {
        use crate::powerquad::sleep::Sleep;

        // Reset the peripheral.
        SystemControl::reset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        // Unreset the peripheral.
        SystemControl::unreset::<Self>();

        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();

        unsafe {
            use core::ptr::write_volatile as write;

            const FORMAT: u32 = (0b10 << 4) | 0b10;

            // Set output formats to F32.
            write((0x400A6000 + 0x004) as *mut u32, FORMAT);
            write((0x400A6000 + 0x00C) as *mut u32, FORMAT);
            write((0x400A6000 + 0x014) as *mut u32, FORMAT);
            write((0x400A6000 + 0x01C) as *mut u32, FORMAT);

            // Enable Event Generation.
            write((0x400A6000 + 0x194) as *mut u32, 0x9F);
        }

        // Register the interrupt.
        unsafe {
            use crate::vtable::{ Vector, VectorTable, };

            // Get a reference to the Vector Table.
            let vtable = &mut *(core::ptr::read_volatile(0xE000ED08 as *const u32) as *mut VectorTable<60>);

            // Register the interrupt.
            vtable.interrupt(57, Vector::handler( engine::handler::handler ), Some(4));
        }

        (Coprocessor::create(), Coprocessor::create(), Engine::create())
    }

    /// Powers off the PowerQuad.
    #[inline(always)]
    fn poweroff() {
        // Disable the clock to the power quad.
        SystemControl::disable::<Self>();
    }

    /// Powers on the PowerQuad.
    #[inline(always)]
    fn poweron() {
        // Enable the clock to the power quad.
        SystemControl::enable::<Self>();
    }
}

impl Control for PowerQuad {
    const REG: u32 = 2;
    const OFF: u8 = 19;
}

impl Enable  for PowerQuad {}
impl Unreset for PowerQuad {}

unsafe impl Disable for PowerQuad {}
unsafe impl Reset   for PowerQuad {}



/// Internal function to turn on the PowerQuad coprocessor.
#[inline(always)]
pub(self) fn poweron() {
    // Decrement the current value and read the last value.
    let _ = POWER.fetch_sub(1, Ordering::AcqRel);

    // Power on.
    //PowerQuad::poweron();
}

/// Internal function to request to turn off the PowerQuad coprocessor.
#[inline(always)]
pub(self) fn poweroff() {
    // Increment the current value and read the last value.
    let last = POWER.fetch_add(1, Ordering::AcqRel);

    // Power off if the other two devices have requested a power off.
    //if last >= 2 { PowerQuad::poweroff(); }
}
