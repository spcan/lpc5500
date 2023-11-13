/* NXP LPC5500 series linker program */



MEMORY {
    FLASH : ORIGIN = 0x00000000, LENGTH = 630K

    RAM : ORIGIN = 0x20000000, LENGTH = 256K
}



SECTIONS {
    /* Vector table region */
    .vector_table :
    {
        /* Force to the start of flash */
        . = ORIGIN(FLASH);

        /* Export Vector Table start address */
        __svt = .;

        /* Reset Stack Pointer */
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        /* Keep the reset vector */
        KEEP(*(.RESETVECTOR));

        /* Keep the initial Vector Table address */
        KEEP(*(.exceptions));

        /* Export Vector Table end address */
        __evt = .;

        . = ORIGIN(FLASH) + 64;
    } > FLASH



    .text ALIGN(8) :
    {
        /* Export text start address */
        __stext = .;

        /* Keep Reset function */
        KEEP(*(Reset));

        /* Include all text */
        *(.text .text.*);

        /* Keep base Hardfault handler */
        *(.Hardfault .Hardfault.*);

        /* Export text end address */
        __etext = .;
    } > FLASH



    .rodata : ALIGN(4)
    {
        /* Align padding */
        . = ALIGN(4);

        /* Export rodata start address */
        __srodata = .;

        /* Include all rodata */
        *(.rodata .rodata.*);

        /* Align padding */
        . = ALIGN(4);

        /* Export rodata end address */
        __erodata = .;
    } > FLASH



    .data : ALIGN(4)
    {
        /* Align the data in flash to 4 bytes */
        . = ALIGN(4);

        /* Export data start address */
        __sdata = .;

        /* Include all data */
        *(.data .data.*);

        /* Align the data in flash to 4 bytes */
        . = ALIGN(4);
    } > RAM AT>FLASH

    /* Align the data to 4 bytes */
    . = ALIGN(4);

    /* Export the data end address */
    __edata = .;

    /* Export the data load addres */
    __sidata = LOADADDR(.data);



    /* Security Attribution Unit blocks */
    .gnu.sgstubs : ALIGN(32)
    {
        /* Align the section to 32 bytes */
        . = ALIGN(32);

        /* Export veneer start address */
        __veneer_base = .;

        /* SGSTUBS */
        *(.gnu.sgstubs);

        /* Align the section to 32 bytes */
        . = ALIGN(32);
    } > FLASH

    /* Align the section to 32 bytes */
    . = ALIGN(32);

    /* Export the veneer end address */
    __veneer_limit = .;



    .bss (NOLOAD) : ALIGN(4)
    {
        /*  Align the section to 4 bytes */
        . = ALIGN(4);

        /* Export the .bss start address */
        __sbss = .;

        *(.bss .bss.*);
        *(COMMON);

        /* Align the VMA of the section to 4 bytes */
        . = ALIGN(4);
    } > RAM

    /* Align the section to 4 bytes */
    . = ALIGN(4);

    /* Export the .bss end address */
    __ebss = .;



    .uninit : ALIGN(4)
    {
        /* Align the data in RAM to 4 bytes */
        . = ALIGN(4);

        /* Export uninit start address */
        __suninit = .;

        /* Include all uninit */
        *(.uninit .uninit.*);

        /* Align the data in RAM to 4 bytes */
        . = ALIGN(4);

        /* Export rodata end address */
        __euninit = .;
    } > RAM



    /* .got section is used just to detect its use */
    .got :
    {
        KEEP(*(.got .got.*));
    }



    /* Discarded sections */
    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
        *(.ARM.extab .ARM.extab.*);
    }
}



/* Assert the size of the vector table is 16 entries * 4 bytes per entry = 64 bytes */
ASSERT( SIZEOF(.vector_table) == 64,
    "BUG(lpc5500-boot): Size of vector table section is not 64 bytes" );

/* Assert FLASH is 4 byte aligned */
ASSERT( ORIGIN(FLASH) % 4 == 0,
    "BUG(lp5500-boot): Flash must be 4 byte aligned" );

/* Assert .data is 4 byte aligned */
ASSERT( (__sdata % 4 == 0) && (__edata % 4 == 0),
    "BUG(lpc5500-boot): .data must be 4 byte aligned" );

/* Assert .data LMA is 4 byte aligned */
ASSERT( __sidata % 4 == 0,
    "BUG(lpc5500-boot): .data LMA must be 4 byte aligned" );

/* Assert .bss is 4 byte aligned */
ASSERT( (__sbss % 4 == 0) && (__ebss % 4 == 0),
    "BUG(lpc5500-boot): .bss must be 4 byte aligned" );

/* Assert no dynamic relocations */
ASSERT( SIZEOF(.got) == 0,
    "ERROR(lpc5500-boot): .got section is not allowed. Dynamic relocations are not
allowed. If linking C code, modify the build without the -fPIC flag." );
