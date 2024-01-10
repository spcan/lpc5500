    /* Reset Vector Table section */
    .vectortable :
    {
        /* Force to the start of flash */
        . = ORIGIN(FLASH);

        /* Export start address */
        __svt = .;

        /* Force keep the Reset Vector Table */
        KEEP(*(.vtable.RESET));

        /* Export end address */
        __evt = .;
    } > FLASH

    /* Code section */
    .text : ALIGN(8)
    {
        /* Export start address */
        __stext = .;

        /* Force keep the Reset function */
        KEEP(*(Reset));

        /* Keep the default Hardfault handler */
        *(.Hardfault .Hardfault.*);

        /* Include all code */
        *(.text .text.*);

        /* Export end address */
        __etext = .;
    } > FLASH

    /* Read-Only Data section */
    .rodata : ALIGN(4)
    {
        /* Align padding */
        . = ALIGN(4);

        /* Export start address */
        __srodata = .;

        /* Include all rodata */
        *(.rodata .rodata.*);

        /* Align padding */
        . = ALIGN(4);

        /* Export end address */
        __erodata = .;
    } > FLASH

    /* Security Attribution Unit blocks region */
    .gnu.sgstubs : ALIGN(32)
    {
        /* Align padding */
        . = ALIGN(32);

        /* Export start addres */
        __veneer_base = .;

        /* Include all data */
        *(.gnu.sgstubs);

        /* Align padding */
        . = ALIGN(32);

        /* Export end addres */
        __veneer_limit = .;
    }

