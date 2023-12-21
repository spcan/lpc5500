//! Defines the number of ADC samples that will be averaged.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AverageCount {
    /// Single conversion without averaging.
    None,

    /// 2 samples averaged.
    Samples2,

    /// 4 samples averaged.
    Samples4,

    /// 8 samples averaged.
    Samples8,

    /// 16 samples averaged.
    Samples16,

    /// 32 samples averaged.
    Samples32,

    /// 64 samples averaged.
    Samples64,

    /// 128 samples averaged.
    Samples128,
}
