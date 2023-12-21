//! Interrupts module.



use super::Vector;



pub(crate) static INTERRUPTS: [Vector; 60] = [
    // WDT, BOD and Flash common interrupt.
    Vector::reserved(),

    // SDMA 0 controller interrupt.
    Vector::reserved(),

    // GPIO group 0 interrupt.
    Vector::reserved(),

    // GPIO group 1 interrupt.
    Vector::reserved(),

    // GPIO pin interrupt 0.
    Vector::reserved(),

    // GPIO pin interrupt 1.
    Vector::reserved(),

    // GPIO pin interrupt 2.
    Vector::reserved(),

    // GPIO pin interrupt 3.
    Vector::reserved(),

    // Micro-tick Timer interrupt.
    Vector::reserved(),

    // Multirate Timer interrupt.
    Vector::reserved(),

    // Standard Counter / Timer 0 interrupt.
    Vector::reserved(),

    // Standard Counter / Timer 1 interrupt.
    Vector::reserved(),

    // SC Timer interrupt.
    Vector::reserved(),

    // Standard Counter / Timer 3 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 0 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 1 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 2 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 3 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 4 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 5 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 6 interrupt.
    Vector::reserved(),

    // Flexcomm Interface 7 interrupt.
    Vector::reserved(),

    // ADC interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Analog Comparator interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // USB 0 Need Clock interrupt.
    Vector::reserved(),

    // USB 0 interrupt.
    Vector::reserved(),

    // RTC interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Wakeup and Mailbox interrupt.
    Vector::reserved(),

    // GPIO pin interrupt 4.
    Vector::reserved(),

    // GPIO pin interrupt 5.
    Vector::reserved(),

    // GPIO pin interrupt 6.
    Vector::reserved(),

    // GPIO pin interrupt 7.
    Vector::reserved(),

    // Standard Counter / Timer 2 interrupt.
    Vector::reserved(),

    // Standard Counter / Timer 4 interrupt.
    Vector::reserved(),

    // OS Event Timer interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // SDIO interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // Reserved interrupt.
    Vector::reserved(),

    // USB 1 PHY interrupt.
    Vector::reserved(),

    // USB 1 interrupt.
    Vector::reserved(),

    // USB 1 Need Clock interrupt.
    Vector::reserved(),

    // Hypervisor interrupt.
    Vector::reserved(),

    // Secure GPIO pin 0 interrupt.
    Vector::reserved(),

    // Secure GPIO pin 1 interrupt.
    Vector::reserved(),

    // Secure Violation interrupt.
    Vector::reserved(),

    // HASH interrupt.
    Vector::reserved(),

    // CASPER Coprocessor interrupt.
    Vector::reserved(),

    // PUF Controller interrupt.
    Vector::reserved(),

    // PowerQuad Coprocessor interrupt.
    Vector::reserved(),

    // Secure DMA 1 Controller interrupt.
    Vector::reserved(),

    // High Speed SPI interrupt.
    Vector::reserved(),

    Vector::reserved(),
];
