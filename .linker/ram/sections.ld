    /* Data section */
    .data : ALIGN(8)
    {
        /* Align the data in FLASH to 8 bytes */
        . = ALIGN(8);

        /* Export start address */
        __sdata = .;

        /* Include all data */
        *(.data .data.*);

        /* Align the data in FLASH to 8 bytes */
        . = ALIGN(4);

        /* Export end address */
        __edata = .;
    } >RAM AT>FLASH

    /* Export the data load address */
    __sidata = LOADADDR(.data);


    /* BSS (Zero initialized) section */
    .bss (NOLOAD) : ALIGN(8)
    {
        /* Align the section to 8 bytes */
        . = ALIGN(8);

        /* Export start address */
        __sbss = .;

        /* Include all BSS and COMMON */
        *(.bss .bss.*);
        *(COMMON);

        /* Align the section to 8 bytes */
        . = ALIGN(8);

        /* Export end address */
        __ebss = .;
    } > RAM


    /* UNINIT section */
    .uninit (NOLOAD) : ALIGN(8)
    {
        /* Align the section to 8 bytes */
        . = ALIGN(8);

        /* Export start address */
        __suninit = .;

        /* Include all UNINIT */
        *(.uninit .uninit.*);

        /* Align the section to 8 bytes */
        . = ALIGN(8);

        /* Export end address */
        __euninit = .;
    } > RAM

