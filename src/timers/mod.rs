//! LPC5500 timers module.
//! Contain all types of internal timers available in the LPC5500 microcontrollers.



pub mod ostimer;
pub mod utick;



pub(super) fn init() -> ostimer::OSTime {
    // Create and initialize the OS timer.
    let ostime = ostimer::OSTimer::create().init();

    ostime
}
