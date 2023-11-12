//! Secutiry features module.



mod random;



pub use random::random;

use crate::system::SystemControl;




pub(super) fn init() {
    // Initialize RNG.
    SystemControl::enable::<random::RNG>();
    SystemControl::unreset::<random::RNG>();
}