




pub struct Trigger<'a>(&'a mut u32);

impl Trigger {
    /// Enables or disables the hardware trigger source.
    pub fn hardware(&mut self, enable: bool) {
        if enable { self.raw  }
    }
}
