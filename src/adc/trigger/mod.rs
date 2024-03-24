//! ADC triggers.



/// Control of an ADC trigger.
pub struct Trigger<const T: u32>(u32);

impl<const T: u32> TriggerConfig for Trigger<T> {
    fn disable(&mut self) {
        unsafe { core::ptr::write_volatile((0x500A00A0 + (T * 4)) as *mut u32, self.0 & !1) }
    }

    fn enable(&mut self) {
        unsafe { core::ptr::write_volatile((0x500A00A0 + (T * 4)) as *mut u32, self.0 | 1) }
    }

    fn fifo(&mut self, a: Buffer, b: Buffer) {
        // Clear the buffer configuration.
        self.0 &= !(0b11 << 1);

        // Get the buffer configuration.
        let cfg = ((a as u32) << 1) | ((b as u32) << 0);

        // Set the buffer configuration.
        self.0 |= cfg << 1;
    }

    fn link<const C: u32>(&mut self, command: &mut Command<C>) {
        // Clear the triggered command.
        self.0 &= !(0xF << 24);

        // Set the new triggered command.
        self.0 |= C << 24;
    }

    fn priority(&mut self, priority: u8) {
        // Clear the priority of the configuration.
        self.0 &= !(0xF << 8);

        // Set the priority of the configuration.
        self.0 |= (priority as u32 & 0xF) << 8;
    }

    fn write(&mut self) {
        unsafe { core::ptr::write_volatile((0x500A00A0 + (T * 4)) as *mut u32, self.0) }
    }
}



/// Common trait for all `Trigger`s.
pub trait TriggerConfig {
    /// Disables the trigger.
    fn disable(&mut self);

    /// Enables the trigger.
    fn enable(&mut self);

    /// Sets the FIFO destination for the trigger results.
    /// `a` sets the destination of the A channel (or the full result in single ended mode).
    /// `b` sets the destination of the B channel.
    fn fifo(&mut self, a: Buffer, b: Buffer);

    /// Links the trigger with the given command slot.
    fn link<const C: u32>(&mut self, command: &mut Command<C>);

    /// Sets the trigger's priority.
    fn priority(&mut self, priority: u8);

    /// Writes the trigger configuration to the peripheral.
    /// Without writing the trigger configuration, it will never activate.
    fn write(&mut self);
}



/// Buffers of the ADC.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Buffer {
    /// ADC FIFO buffer 0.
    FIFO0 = 0,

    /// ADC FIFO buffer 1.
    FIFO1 = 1,
}
