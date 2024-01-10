//! Interrupt module.



pub(crate) mod exceptions;
pub(crate) mod global;
pub(crate) mod interrupts;

mod table;
mod vector;



pub use table::VectorTable;
pub use vector::Vector;

pub(self) use exceptions::{ Block, Ignore, };



/// Initializes the Vector Tables of all cores.
pub(super) fn init() {
    extern "C" {
        static __estack0: u32;
    }

    // Register Core 0 exceptions.
    unsafe { global::main::CORE0VTABLE.copyexc( &global::reset::RESETVTABLE ) }

    // Set up Core 0 NVIC.
    vtableinit( unsafe { &interrupts::main::C0INTERRUPTS } );
}


/// Common function for all cores to initialize the vector table.
pub extern "Rust" fn vtableinit(vtable: &VectorTable<60>) {
    // Modify VTOR register.
    unsafe { core::ptr::write_volatile(0xE000ED08 as *mut u32, vtable as *const _ as u32) }

    // Initialize NVIC.
}
