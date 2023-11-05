//! Clock information of the internal oscillators.



use core::sync::atomic::{
    AtomicU32, Ordering,
};



pub struct ClockInformation {
    /// Status of the internal 1 MHz FRO.
    pub(super) fro1m : AtomicU32,

    /// Status of the internal 12 MHz FRO.
    pub(super) fro12m: AtomicU32,

    /// Status of the internal 48 MHz FRO.
    pub(super) fro48m: AtomicU32,

    /// Status of the internal 96 MHz FRO.
    pub(super) fro96m: AtomicU32,

    /// Status of the internal 32 kHz FRO.
    pub(super) fro32k: AtomicU32,
}

impl ClockInformation {
    /// Static initializer.
    pub const fn new() -> Self {
        Self {
            fro1m : AtomicU32::new(0),
            fro12m: AtomicU32::new(0),
            fro48m: AtomicU32::new(0),
            fro96m: AtomicU32::new(0),
            fro32k: AtomicU32::new(0),
        }
    }

    /// Returns the last known state of the 1 MHz FRO clock.
    pub fn fro1m(&self) -> u32 {
        self.fro1m.load( Ordering::Relaxed )
    }

    /// Returns the last known state of the 12 MHz FRO clock.
    pub fn fro12m(&self) -> u32 {
        self.fro12m.load( Ordering::Relaxed )
    }

    /// Returns the last known state of the 48 MHz FRO clock.
    pub fn fro48m(&self) -> u32 {
        self.fro48m.load( Ordering::Relaxed )
    }

    /// Returns the last known state of the 96 MHz FRO clock.
    pub fn fro96m(&self) -> u32 {
        self.fro96m.load( Ordering::Relaxed )
    }

    /// Returns the last known state of the 32 kHz FRO clock.
    pub fn fro32k(&self) -> u32 {
        self.fro32k.load( Ordering::Relaxed )
    }
}
