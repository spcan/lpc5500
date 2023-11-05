//! Internal and external oscillators module.



mod fro;


pub struct ClockControl {
    /// The 32 kHz FRO signal control.
    fro32k: fro::FRO32KHz,

    /// The 1 MHz FRO signal control.
    fro1m: fro::FRO1MHz,

    /// The 12 MHz FRO signal control.
    fro12m: fro::FRO12MHz,

    /// The 48 MHz FRO signal control.
    fro48m: fro::FRO48MHz,

    /// The 96 MHz FRO signal control.
    fro96m: fro::FRO96MHz,
}
