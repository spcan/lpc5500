//! Flash configuration.



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};



pub struct Flash {

}

impl Flash {
    /// Base address of the SYSCON peripheral.
    const SYSCON: u32 = 0x50000000;

    /// Register offset of the FMCCR register.
    const FMCCR: u32 = 0x400;

    /// Base address of the FLASH peripheral.
    const FLASH: u32 = 0x40034000;

    /// Register offset of the CMD register.
    const CMD: u32 = 0x000;

    /// Register offset of the DATAW0 register.
    const DATAW0: u32 = 0x80;

    /// Register offset of the INTSTATUS register.
    const INTSTATUS: u32 = 0xFE0;

    /// Register offset of the INTCLRSTATUS register.
    const INTCLRSTATUS: u32 = 0xFE8;

    /// Configures the Flash acceleration.
    pub fn acceleration(&mut self, config: AcceleratonConfig) {
        // FMCCR pointer.
        let ptr = Self::SYSCON + Self::FMCCR;

        // Read the register.
        let mut fmccr = unsafe { read(ptr as *const u32) };

        // Clear all data that will be modified.
        fmccr &= !0x7F;

        // Configure the cache buffers.
        fmccr |= match (config.ibuf, config.dbuf) {
            // Set both Data and Instructions to use one buffer.
            (BufferUsage::All, BufferUsage::All) | (BufferUsage::One, BufferUsage::One) |
            (BufferUsage::All, BufferUsage::One) | (BufferUsage::One, BufferUsage::All)   => (0x1 << 2) | (0x1 << 0),

            // Set Instruction to use all buffers.
            (BufferUsage::All, BufferUsage::None) | (BufferUsage::One, BufferUsage::None) => (0x0 << 2) | (0x2 << 0),

            // Set Data to use all buffers.
            (BufferUsage::None, BufferUsage::All) | (BufferUsage::None, BufferUsage::One) => (0x2 << 2) | (0x0 << 0),

            // Disable cache buffers.
            (BufferUsage::None, BufferUsage::None) => (0x0 << 2) | (0x0 << 0),
        };

        // Enable acceleration if set.
        if config.enabled { fmccr |= 1 << 4; }

        // Enable prefetch if set.
        if config.prefetch { fmccr |= 1 << 5; }

        // Write the modified FMCCR register.
        unsafe { write(ptr as *mut u32, fmccr) }
    }

    /// Configures the Flash wait states for the given frequency.
    pub(super) fn configfreq(&mut self, frequency: u32) {
        // Get the number of wait states.
        let wait = Self::waitstates(frequency);

        // FMCCR pointer.
        let ptr = Self::SYSCON + Self::FMCCR;

        // Read the current state of the register.
        let restore = unsafe { read(ptr as *const u32) };

        // Clear the prefetch bit.
        unsafe { write(ptr as *mut u32, restore & !(1 << 5)) }

        // Clear all status flags.
        self.clearall();

        // Write to DATAW0.
        self.wdata::<0>( (self.rdata::<0>() & 0xFFFFFFF0) | waitstates );

        // Set read mode.
        self.command( 0x2 );

        // Wait for completion.
        while self.status() == 0 {
            unsafe { core::arch::asm!("nop", options(nostack, nomem)) }
        }

        // Set the FMCCR register again with the new wait states.
        unsafe { write(ptr as *mut u32, (restore & !(0x1F << 12)) | (wait << 12) ) }
    }

    fn command(&mut self, command: u32) {
        // Get the pointer.
        let dst: u32 = Self::FLASH + Self::CMD;

        // Write the command.
        unsafe { write( dst as *mut u32, command ) }
    }

    fn status(&mut self) -> u32 {
        // Get the pointer.
        let src: u32 = Self::FLASH + Self::INTSTATUS;

        // Write the command.
        unsafe { read( src as *const u32 ) }
    }

    fn wdata<const N: usize>(&mut self, src: u32) where [(); 4-N]: Sized {
        // Get the pointer.
        let dst = Self::FLASH + Self::DATAW0 + (N as u32 * 4);

        // Write the data.
        unsafe { write( dst as *mut u32, src ) }
    }

    fn rdata<const N: usize>(&mut self) -> u32 where [(); 4-N]: Sized {
        // Get the pointer.
        let src = Self::FLASH + Self::DATAW0 + (N as u32 * 4);

        // Read the data.
        unsafe { read( src as *mut u32 ) }
    }

    /// Clears all status flags.
    fn clearall(&mut self) {
        unsafe { write( (Self::FLASH + Self::INTCLRSTATUS) as *mut u32, 0x1F ) }
    }

    /// Calculates the number of wait states for a frequency.
    fn waitstates(frequency: u32) -> u32 {
        match frequency {
                      0..= 11_000_000 =>  0,
             11_000_001..= 22_000_000 =>  1,
             22_000_001..= 33_000_000 =>  2,
             33_000_001..= 44_000_000 =>  3,
             44_000_001..= 55_000_000 =>  4,
             55_000_001..= 66_000_000 =>  5,
             66_000_001..= 77_000_000 =>  6,
             77_000_001..= 88_000_000 =>  7,
             88_000_001..=100_000_000 =>  8,
            100_000_001..=115_000_000 =>  9,
            115_000_001..=130_000_000 => 10,
            130_000_001..=150_000_000 => 11,
            _                         => 12
        }
    }
}



/// Flash acceleration configuration.
pub struct AccelerationConfig {
    /// Indicates if performance is enabled.
    /// Disabling acceleration allows for more determinism at the cost of performance.
    pub enabled: bool,

    /// Usage of buffers for data accesses.
    pub dbuf: BufferUsage,

    /// Usage of buffers for instruction accesses.
    pub ibuf: BufferUsage,

    /// Enables prefetching the next Flash line for instruction accesses.
    pub prefetch: bool,
}



/// Buffer usage strategy for Flash accesses (instruction and data).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BufferUsage {
    /// No buffers are used for Flash accesses (instruction or data).
    None,

    /// One buffer is used for Flash accesses (instruction or data).
    One,

    /// All buffers are used for Flash accesses (instruction or data).
    All,
}
