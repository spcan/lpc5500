//! Free Running Oscillator module.



mod fro1m;
mod fro192m;
mod fro32k;



/// Initialize all the FRO clock signals.
pub(super) fn init() {
    // ANACTRL peripheral base.
    const ANACTRL: usize = 0x50013000;

    // FRO192MCTRL register offset.
    const FRO192MCTRL: usize = 0x10;

    // PMC peripheral base.
    const PMC: usize = 0x40020000;

    // PDRUN register offset.
    const PDRUN: usize = 0xB8;

    // PDRUNCLR register offset.
    const PDRUNCLR: usize = 0xC8;

    // Start the 192 MHz FRO [PDRUN bit 5] and 32 kHz FRO [PDRUN bit 6].
    fro192m::FRO192MHz::init();
    fro32k::FRO32kHz::init();

    // Enable the 1 Mhz FRO.
    fro1m::FRO1MHz::init();

    // Log the register states.
    unsafe {
        // Read the PDRUN register.
        let pdrun = core::ptr::read_volatile((PMC + PDRUN) as *const u32);

        let fen = ((pdrun >> 5) & 1) == 0;
        let sen = ((pdrun >> 5) & 1) == 0;

        defmt::debug!("192 MHz FRO is powered: {} | 32 kHz FRO is powered: {}", fen, sen);

        // Read the FRO192M_CTRL register.
        let froen = core::ptr::read_volatile((ANACTRL + FRO192MCTRL) as *const u32);

        let en12 = ((froen >> 14) & 1) == 1;
        let en48 = ((froen >> 15) & 1) == 1;
        let en96 = ((froen >> 30) & 1) == 1;

        defmt::debug!("12 Mhz FRO is enabled: {} | 48 Mhz FRO is enabled: {} | 96 Mhz FRO is enabled: {}", en12, en48, en96);
    }
}
