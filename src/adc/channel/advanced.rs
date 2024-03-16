//! Advanced ADC channel usage.
//! Channels 0 to 4 implement this functionality.



use super::{
    ADCChannel, Channel,

    super::{
        Command,

        common::*,
    },
};



/// Common trait for the advanced functionality of and ADC channel.
pub trait ADCAdvanced: ADCChannel {
    /// Creates an advanced command with the given measurement configuration.
    /// - `config` provides the ADC conversion configuration.
    /// - `cmp` provides the conditional store configuration.
    /// - `wait` makes the ADC wait a preconfigured (by the ADC) delay between chained commands.
    /// </div class="warning">`wait` does not apply if this is the first command.</div>
    fn command(&mut self, config: Conversion, cmp: Option<Compare>, wait: bool) -> Command {

    }
}


pub struct Conversion {
    /// The measurement mode.
    mode: Mode,

    /// The resolution of the measurement.
    resolution: Resolution,

    /// The number of averaged measurements.
    average: Average,

    /// The sample time of each measurement.
    time: SampleTime,
}


impl ADCAdvanced for Channel<0> {}
impl ADCAdvanced for Channel<1> {}
impl ADCAdvanced for Channel<2> {}
impl ADCAdvanced for Channel<3> {}
impl ADCAdvanced for Channel<4> {}
