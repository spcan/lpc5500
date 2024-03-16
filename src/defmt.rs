//! `defmt` framework configuration.



/// Initializes the `defmt` framework for the chip.
pub(super) fn init() {
    // Define the `defmt` timestamp.
    defmt::timestamp!("{=u64:us}", (crate::OSTIME.read() * 1_000_000) / 32768);
}
