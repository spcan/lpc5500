//! Interrupt module.



pub(crate) mod exceptions;
pub(crate) mod global;

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
    unsafe { global::runtime::CORE0VTABLE.copyexc( &global::reset::RESETVTABLE ) }

    // Set up Core 0 NVIC.
    vtableinit( unsafe { &global::runtime::CORE0VTABLE } );
}



/// Common function for all cores to initialize the vector table.
pub extern "Rust" fn vtableinit(vtable: &VectorTable<60>) {
    // Modify VTOR register.
    unsafe { core::ptr::write_volatile(0xE000ED08 as *mut u32, vtable as *const _ as u32) }

    // Initialize NVIC.
}



#[cfg(feature = "defmt")]
#[doc = "Logs over `defmt` the current state of the Vector Tables."]
pub fn report() {
    // Log the Vector Tables.
    defmt::info!("CORE 0 {}", unsafe { &global::runtime::CORE0VTABLE } );
    defmt::info!("CORE 1 {}", unsafe { &global::runtime::CORE1VTABLE } );
}