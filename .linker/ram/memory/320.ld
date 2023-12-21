    /* SRAMX region */
    SRAMX(rwx) : ORIGIN = 0x04000000, LENGTH = 32K

    /* SRAM 0 + SRAM 1 + SRAM 2 + SRAM 3 region */
    RAM(rwx) : ORIGIN = ORIGIN(STACK1) + LENGTH(STACK1), LENGTH = 256K - LENGTH(STACK0) - LENGTH(STACK1)

    /* PowerQuad RAM region */
    PQRAM(rw) : ORIGIN = 0x20040000, LENGTH = 16K

    /* USB RAM region */
    USBRAM(rw) : ORIGIN = 0x40100000, LENGTH = 16K
