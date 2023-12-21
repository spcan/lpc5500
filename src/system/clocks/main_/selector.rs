//! Abstraction over the Main Clock Selector A clock signal.



use core::sync::atomic::{
    AtomicBool, AtomicUsize, Ordering,
};



/// Number of references of the Main Clock Selector A clock signal.
#[link_section = ".data.lpc5500.clocks.MCLOCKSELA"]
static REFCNT: AtomicUsize = AtomicUsize::new( 0 );

/// Current status of the Main Clock Selector A clock signal.
#[link_section = ".data.lpc5500.clocks.MCLOCKSELA"]
static ENABLE: AtomicBool = AtomicBool::new( false );



pub struct MainClockSelector {
    /// The current source of the Main Clock Selector A.
    source: Source,
}


impl MainClockSelector {
    /// Sets the clock source for the Main Clock Selector A.
    /// Returns `true` if the operation was successful.
    fn from<S: MainClockSelectorSource>(&mut self, source: S) -> bool {
        // Check if the clock is already locked.
        if REFCNT.load( Ordering::Relaxed ) > 0 { return false }

        // Modify the source register.

        // Set the source tag.
        self.source = S::Source;
    }
}


/// Common trait for all clock sources of the MainClockSelector.
pub trait MainClockSelectorSource {
    
}
