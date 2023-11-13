//! FRO clock signals' implementations.



use core::sync::atomic::{
    AtomicBool, AtomicUsize, Ordering,
};

use super::{
    FROControl,

    FROEnable, FRODisable,

    ClockSource, ClockEnableControl, ClockDisableControl,
};



/// Number of references of the FRO 12 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO12M"]
static FRO12MREFCNT: AtomicUsize = AtomicUsize::new( 0 );

/// Current status of the FRO 12 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO12M"]
static FRO12MENABLE: AtomicBool = AtomicBool::new( false );



/// FRO 12 MHz output clock signal.
pub struct FRO12MHz;

impl ClockSource for FRO12MHz {
    fn current() -> u32 {
        if FRO12MENABLE.load( Ordering::Relaxed ) { 12_000_000 } else { 0 }
    }

    fn lock(&mut self) -> bool {
        // If the clock is not enabled it cannot lock.
        if !FRO12MENABLE.load( Ordering::Relaxed ) { return false }

        // Increase the reference count of the clock.
        FRO12MREFCNT.fetch_add(1, Ordering::Relaxed);

        // Indicate that the lock was successful.
        return true;
    }
}

impl ClockEnableControl for FRO12MHz {
    fn enable(&mut self) {
        // If already enabled return early.
        if FRO12MENABLE.load( Ordering::Relaxed ) { return }

        // Enable the clock.
        FROControl::enable::<Self>();

        // Mark as enabled.
        FRO12MENABLE.store(true, Ordering::Relaxed);
    }
}

impl ClockDisableControl for FRO12MHz {
    fn disable(&mut self) -> bool {
        // Lock the enable register.
        let enable = FRO12MENABLE.load( Ordering::Relaxed );

        // If already disabled return.
        if !enable { return true }

        // Read if the clock is being used.
        if FRO12MREFCNT.load( Ordering::Relaxed ) == 0 {
            // Disable the clock.
            FROControl::disable::<Self>();

            // Mark as disabled.
            FRO12MENABLE.store(false, Ordering::Relaxed);

            return true;
        }

        return false;
    }
}

impl FROEnable for FRO12MHz {
    const OFFSET: u8 = 14;
}

impl FRODisable for FRO12MHz {}



/// Number of references of the FRO 48 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO48M"]
static FRO48MREFCNT: AtomicUsize = AtomicUsize::new( 0 );

/// Current status of the FRO 48 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO48M"]
static FRO48MENABLE: AtomicBool = AtomicBool::new( false );



/// FRO 48 MHz output clock signal.
pub struct FRO48MHz;

impl ClockSource for FRO48MHz {
    fn current() -> u32 {
        if FRO48MENABLE.load( Ordering::Relaxed ) { 12_000_000 } else { 0 }
    }

    fn lock(&mut self) -> bool {
        // If the clock is not enabled it cannot lock.
        if !FRO48MENABLE.load( Ordering::Relaxed ) { return false }

        // Increase the reference count of the clock.
        FRO48MREFCNT.fetch_add(1, Ordering::Relaxed);

        // Indicate that the lock was successful.
        return true;
    }
}

impl ClockEnableControl for FRO48MHz {
    fn enable(&mut self) {
        // If already enabled return early.
        if FRO48MENABLE.load( Ordering::Relaxed ) { return }

        // Enable the clock.
        FROControl::enable::<Self>();

        // Mark as enabled.
        FRO48MENABLE.store(true, Ordering::Relaxed);
    }
}

impl FROEnable for FRO48MHz {
    const OFFSET: u8 = 15;
}



/// Number of references of the FRO 96 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO96M"]
static FRO96MREFCNT: AtomicUsize = AtomicUsize::new( 0 );

/// Current status of the FRO 96 MHz clock.
#[link_section = ".data.lpc5500.clocks.FRO96M"]
static FRO96MENABLE: AtomicBool = AtomicBool::new( false );



/// FRO 96 MHz output clock signal.
pub struct FRO96MHz;

impl ClockSource for FRO96MHz {
    fn current() -> u32 {
        if FRO96MENABLE.load( Ordering::Relaxed ) { 12_000_000 } else { 0 }
    }

    fn lock(&mut self) -> bool {
        // If the clock is not enabled it cannot lock.
        if !FRO96MENABLE.load( Ordering::Relaxed ) { return false }

        // Increase the reference count of the clock.
        FRO96MREFCNT.fetch_add(1, Ordering::Relaxed);

        // Indicate that the lock was successful.
        return true;
    }
}

impl ClockEnableControl for FRO96MHz {
    fn enable(&mut self) {
        // If already enabled return early.
        if FRO96MENABLE.load( Ordering::Relaxed ) { return }

        // Enable the clock.
        FROControl::enable::<Self>();

        // Mark as enabled.
        FRO96MENABLE.store(true, Ordering::Relaxed);
    }
}

impl ClockDisableControl for FRO96MHz {
    fn disable(&mut self) -> bool {
        // Lock the enable register.
        let enable = FRO96MENABLE.load( Ordering::Relaxed );

        // If already disabled return.
        if !enable { return true }

        // Read if the clock is being used.
        if FRO96MREFCNT.load( Ordering::Relaxed ) == 0 {
            // Disable the clock.
            FROControl::disable::<Self>();

            // Mark as disabled.
            FRO96MENABLE.store(false, Ordering::Relaxed);

            return true;
        }

        return false;
    }
}

impl FROEnable for FRO96MHz {
    const OFFSET: u8 = 30;
}

impl FRODisable for FRO96MHz {}
