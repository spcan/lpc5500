//! DMA configuration register.





pub struct ChannelConfig(pub(super) u32);

impl ChannelConfig {
    /// Enables / Disables hardware trigger.
    pub fn hwtrigger(mut self, enable: bool) -> Self {
        if enable { self.0 |=  (1 << 1) }
        else      { self.0 &= !(1 << 1) }

        self
    }

    /// Sets the trigger type of the DMA channel.
    pub fn trigger(mut self, trigger: Trigger) -> Self {
        match trigger {
            Trigger::Edge  => self.0 &= !(1 << 1),
            Trigger::Level => self.0 |=  (1 << 1),
        }
    }

    
}
