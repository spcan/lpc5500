//! Flash control.



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};



pub struct FlashControl;

impl FlashControl {
    /// Base address of the SYSCON peripheral.
    const SYSCON: u32 = 0x50000000;

    /// Register address of the FMCCR register.
    const FMCCR: u32 = Self::SYSCON + 0x400;

    /// Base address of the FLASH peripheral.
    const FLASH: u32 = 0x40034000;

    /// Command register offset.
    const CMD: u32 = Self::FLASH + 0x000;

    /// Data word 0 register offset.
    const DATAW0: u32 = Self::FLASH + 0x80;

    /// Interrupt Enable register offset.
    const INTENABLE: u32 = Self::FLASH + 0xFE4;

    /// Interrupt Enable Clear register offset.
    const INTENABLECLR: u32 = Self::FLASH + 0xFD8;

    /// Interrupt Enable Set register offset.
    const INTENABLESET: u32 = Self::FLASH + 0xFDC;

    /// Interrupt Status register offset.
    const INTSTATUS: u32 = Self::FLASH + 0xFE0;

    /// Interrupt Status Clear register offset.
    const INTSTATUSCLR: u32 = Self::FLASH + 0xFE8;

    /// Interrupt Status Set register offset.
    const INTSTATUSSET: u32 = Self::FLASH + 0xFEC;

    /// Initializes the Flash control system.
    pub(crate) fn init() -> Self {
        Self
    }

    /// Configures the Flash acceleration.
    /// Setting the acceleration configuration to `None` disables it.
    pub fn acceleration(&mut self, config: Option<Acceleration>) {
        // Read the FMCCR register.
        let mut fmccr = unsafe { read(Self::FMCCR as *const u32) };

        // Disable the Flash acceleration.
        fmccr &= !(1 << 4);

        // Unpack the configuration.
        if let Some(config) = config {
            // Clear all data that will be modified.
            fmccr &= !0x7F;

            // Configure the cache buffers.
            fmccr |= ((config.dbuf as u32) << 2) | ((config.ibuf as u32) << 0);

            // Enable prefetch if set.
            if config.prefetch { fmccr |= 1 << 5; }

            // Enable acceleration.
            fmccr |= 1 << 4;
        }

        // Write the FMCCR register.
        unsafe { write(Self::FMCCR as *mut u32, fmccr) }
    }

    /// Configures the Flash for the given frequency.
    pub(crate) fn target(&mut self, frequency: u32) {
        // Get the number of wait states.
        let wait = Self::waitstates(frequency);

        //defmt::debug!("Setting {} waitstates for {} MHz", wait, frequency / 1_000_000);

        // Read the current state of the FMCCR.
        let fmccr = unsafe { read(Self::FMCCR as *const u32) };

        // Disable PREFETCH acceleration.
        unsafe { write(Self::FMCCR as *mut u32, fmccr & !(1 << 5)) }

        // Clear all status flags.
        self.clearall();

        // Write to DATAW0 the number of waitstates.
        let data = self.rdata::<0>() & 0xFFFFFFF0;
        self.wdata::<0>( data | wait );

        // Set the read mode command.
        self.command( 0x2 );

        // Wait for completion.
        while self.status() == 0 {
            unsafe { core::arch::asm!("nop", options(nostack, nomem)) }
        }

        // Write the modified FMCCR.
        unsafe { write(Self::FMCCR as *mut u32, (fmccr & !(0x1F << 12)) | (wait << 12)) }
    }

    /// Clears all status flags.
    fn clearall(&mut self) {
        unsafe { write(Self::INTSTATUSCLR as *mut u32, 0x1F) }
    }

    /// Writes a command to the Flash controller.
    fn command(&mut self, command: u32) {
        unsafe { write( Self::CMD as *mut u32, command ) }
    }

    /// Reads the status register.
    fn status(&self) -> u32 {
        unsafe { read(Self::INTSTATUS as *const u32) }
    }

    /// Writes a data word into its register.
    fn rdata<const N: usize>(&mut self) -> u32 where [(); 4-N]: Sized {
        unsafe { read( (Self::DATAW0 + (N as u32 * 4)) as *const u32 ) }
    }

    /// Writes a data word into its register.
    fn wdata<const N: usize>(&mut self, src: u32) where [(); 4-N]: Sized {
        unsafe { write( (Self::DATAW0 + (N as u32 * 4)) as *mut u32, src ) }
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
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature="defmt", derive(defmt::Format))]
pub struct Acceleration {
    /// Usage of buffers for data accesses.
    pub dbuf: BufferUsage,

    /// Usage of buffers for instruction accesses.
    pub ibuf: BufferUsage,

    /// Enables prefetching the next Flash line for instruction accesses.
    pub prefetch: bool,
}



/// Buffer usage strategy for Flash accesses (instruction and data).
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature="defmt", derive(defmt::Format))]
pub enum BufferUsage {
    /// No buffers are used for Flash accesses (instruction or data).
    None,

    /// Only one buffer is used for Flash accesses (instruction or data).
    One,

    /// All buffers is used for Flash accesses (instruction or data).
    All,
}
