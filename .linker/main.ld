

/* Memory map visible by the main core (CORE 0) */
MEMORY {
    /* Flash memory - Non secure - Code bus */
    FLASH(RWX) : ORIGIN = 0x00000000, LENGTH = 630K

    /* Boot ROM - Non secure - Code bus */
    ROM(RX) : ORIGIN = 0x03000000, LENGTH = 128K

    /* SRAM - Non secure - Data bus */
    RAM(RWX) : ORIGIN = 0x20000000, LENGTH = 256K
}
