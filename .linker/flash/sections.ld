    /* Reset Vector Table section */
    .vector_table :
    {
        /* Force to the start of flash */
        . = ORIGIN(FLASH);

        /* Export start address */
        __svt = .;

        /* Reset Stack Pointer */
        LONG( ORIGIN(RAM) + LENGTH(RAM) );

        /* Force keep the Reset Vector */
        KEEP(*(.RESETVECTOR));

        /* Force keep the Reset Vector Table */
        KEEP(*(.exceptions));

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

