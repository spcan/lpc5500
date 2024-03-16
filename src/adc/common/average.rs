//! Number of samples to average in an ADC conversion.



#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Average {
    /// 2 samples averaged.
    Samples2 = 1,

    /// 4 samples averaged.
    Samples4 = 2,

    /// 8 samples averaged.
    Samples8 = 3,

    /// 16 samples averaged.
    Samples16 = 4,

    /// 32 samples averaged.
    Samples32 = 5,

    /// 64 samples averaged.
    Samples64 = 6,

    /// 128 samples averaged.
    Samples128 = 7,
}
