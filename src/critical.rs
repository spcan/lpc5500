//! Critical section implementation.



use core::{
    ptr::{
        read_volatile as read,
        write_volatile as write,
    },

    sync::atomic::{
        AtomicU8, Ordering,
    },
};

use critical_section::{
    set_impl,

    Impl, RawRestoreState,
};



/// Tag of who owns the lock.
#[link_section = ".bss.critical-section.OWNER"]
static mut OWNER: AtomicU8 = AtomicU8::new(0);

/// Flag that indicates that the lock is already owned.
const PREOWNED: u8 = 3;

/// Flag that indicates that the lock is not owned.
const UNOWNED: u8 = 0;



/// Critical section for the dual core environment.
struct DualCoreCriticalSection;

set_impl!(DualCoreCriticalSection);

unsafe impl Impl for DualCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // Read PRIMASK.
        let active = active();

        // Read the core ID.
        // This is done by identifying if the FPU is active, as only Core 0 has an FPU.
        let core = id() + 1;

        // Is this lock already owned.
        if OWNER.load( Ordering::Acquire ) == core {
            return PREOWNED;
        }

        // Spin until we get the lock.
        loop {
            // Disable interrupts to ensure that we won't deadlock.
            core::arch::asm!("cpsid if");

            // Ensure the compiler doesn't reorder accesses.
            core::sync::atomic::compiler_fence( Ordering::SeqCst );

            // Read the spin lock.
            if read( 0x5008B0F8 as *const u32 ) != 0 {
                // Store which core we are.
                OWNER.store(core, Ordering::Relaxed);

                // Stop the loop.
                break;
            }

            // No spinlock, enable interrupts.
            if active {
                core::arch::asm!("cpsie if");
            }
        }

        active as _
    }

    unsafe fn release(token: RawRestoreState) {
        // Did this already own the lock at the start of the section?
        if token != PREOWNED {
            // It wasn't owned at the start of this critical section.
            // Set owner back to unowned.
            OWNER.store( UNOWNED, Ordering::Relaxed );

            // Ensure the compiler doesn't reorder accesses.
            core::sync::atomic::compiler_fence( Ordering::SeqCst );

            // Release the spinlock.
            write( 0x5008B0F8 as *mut u32, 1 );

            // Re-enable interrupts if they were enabled when acquired.
            if token != 0 {
                // Enable interrupts.
                core::arch::asm!("cpsie if");
            }
        }
    }
}



/// Returns the core ID by checking for an FPU.
fn id() -> u8 {
    match unsafe { (read(0xE000ED88 as *const u32) >> 20) & 0b11 } {
        0 => 1,
        _ => 0,
    }
}



/// Returns `true` if PRIMASK was active.
fn active() -> bool {
    let r: u32;

    unsafe { core::arch::asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };

    !(r & (1 << 0) == (1 << 0))
}
