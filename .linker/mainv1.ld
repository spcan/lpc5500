

EXTERN(ResetMain);
ENTRY(ResetMain);



/* Memory map visible by the main core (CORE 0) */
MEMORY {
    /* Flash memory - Non secure - Code bus */
    FLASH(RWX) : ORIGIN = 0x00000000, LENGTH = 630K

    /* Flash memory - Secure - Code bus */
    SECUREFLASH(RWX) : ORIGIN = 0x10000000, LENGTH = 630K



    /* Boot ROM - Non secure - Code bus */
    ROM(RX) : ORIGIN = 0x03000000, LENGTH = 128K

    /* Boot ROM - Secure - Code bus */
    SECUREROM(RX) : ORIGIN = 0x13000000, LENGTH = 128K


    /* SRAMX 0 - Non secure - Data bus */
    SRAMX0(RWX) : ORIGIN = 0x04000000, LENGTH = 4K

    /* SRAMX 0 - Secure - Data bus */
    SECURESRAMX0(RWX) : ORIGIN = 0x14000000, LENGTH = 4K

    /* SRAMX 1 - Non secure - Data bus */
    SRAMX1(RWX) : ORIGIN = 0x04004000, LENGTH = 4K

    /* SRAMX 1 - Secure - Data bus */
    SECURESRAMX1(RWX) : ORIGIN = 0x14004000, LENGTH = 4K

    /* SRAMX 2 - Non secure - Data bus */
    SRAMX2(RWX) : ORIGIN = 0x04006000, LENGTH = 1536

    /* SRAMX 2 - Secure - Data bus */
    SECURESRAMX2(RWX) : ORIGIN = 0x14006000, LENGTH = 1536



    /* SRAM 0 - Non secure - Data bus */
    SRAM0(RWX) : ORIGIN = 0x20000000, LENGTH = 64K

    /* SRAM 0 - Secure - Data bus */
    SECURESRAM0(RWX) : ORIGIN = 0x30000000, LENGTH = 64K

    /* SRAM 1 - Non secure - Data bus */
    SRAM1(RWX) : ORIGIN = 0x20010000, LENGTH = 64K

    /* SRAM 1 - Secure - Data bus */
    SECURESRAM1(RWX) : ORIGIN = 0x30010000, LENGTH = 64K

    /* SRAM 2 - Non secure - Data bus */
    SRAM2(RWX) : ORIGIN = 0x20020000, LENGTH = 64K

    /* SRAM 2 - Secure - Data bus */
    SECURESRAM2(RWX) : ORIGIN = 0x30020000, LENGTH = 64K

    /* SRAM 3 - Non secure - Data bus */
    SRAM3(RWX) : ORIGIN = 0x20030000, LENGTH = 64K

    /* SRAM 3 - Secure - Data bus */
    SECURESRAM3(RWX) : ORIGIN = 0x30030000, LENGTH = 64K

    /* SRAM 4 - Non secure - Data bus */
    SRAM4(RWX) : ORIGIN = 0x20040000, LENGTH = 16K

    /* SRAM 4 - Secure - Data bus */
    SECURESRAM4(RWX) : ORIGIN = 0x30040000, LENGTH = 16K



    /* AHB to APB Bridge 0 - Non secure - Data bus */
    BRIDGE0(RW) : ORIGIN = 0x40000000, LENGTH = 128K

    /* AHB to APB Bridge 0 - Non secure - Data bus */
    SECUREBRIDGE0(RW) : ORIGIN = 0x50000000, LENGTH = 128K

    /* AHB to APB Bridge 1 - Non secure - Data bus */
    BRIDGE1(RW) : ORIGIN = 0x40020000, LENGTH = 128K

    /* AHB to APB Bridge 1 - Non secure - Data bus */
    SECUREBRIDGE1(RW) : ORIGIN = 0x50020000, LENGTH = 128K



    /* USB SRAM - Non secure - Data bus */
    USBSRAM(RW) : ORIGIN = 0x40100000, LENGTH = 16K

    /* USB SRAM - Secure - Data bus */
    SECUREUSBSRAM(RW) : ORIGIN = 0x50100000, LENGTH = 16K
}

SECTIONS {
    /* Vector Table and Image validation */
    .vector_table ORIGIN(FLASH) :
    {
        /* Initial Stack Pointer - 0x00 */
        KEEP(*(.vtable.spmain));

        /* Initial Program Counter - 0x04 */
        KEEP(*(.vtable.pcmain));

        /* Exception Vector Table - First part - 0x08 */
        KEEP(*(.vtable.nmi));
        KEEP(*(.vtable.hard));
        KEEP(*(.vtable.mem));
        KEEP(*(.vtable.bus));
        KEEP(*(.vtable.usage));
        KEEP(*(.vtable.secure));

        /* Image Length - 0x20 */
        /* KEEP(*(.boot.ilength)); */
        LONG(0x00000000);

        /* Image Type - 0x24 */
        /*KEEP(*(.boot.itype)); */
        LONG(0x00000000);

        /* Offset to Security Header or CRC Checksum - 0x28 */
        /*KEEP(*(.boot.hoffset)); */
        LONG(0x00000000);

        /* Exception Vector Table - Second part - 0x2C */
        KEEP(*(.vtable.svcall));
        KEEP(*(.vtable.debug));

        /* Image Execution Address - 0x34 */
        /*KEEP(*(.boot.iea));*/
        LONG(0x00000000);

        /* Exception Vector Table - Third part - 0x38 */
        KEEP(*(.vtable.pendsv));
        KEEP(*(.vtable.systick));

        . += 256;
    } > FLASH

    /* Code section */
    .text : ALIGN(4)
    {
        /* Reset code. */
        KEEP(*(.Reset));
        KEEP(*(.Reset.*));

        /* User code */
        *(.text .text.*);

        . = ALIGN(4);
    } > FLASH



    /* Static data */
    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*);;

        . = ALIGN(4);
    } > FLASH



    /* Data section */
    .data : ALIGN(4)
    {
        __sdata = ALIGN(4);

        *(.data .data.*);
    } > SRAM0 AT > FLASH

    __edata = ALIGN(4);
    __ldata = LOADADDR(.data);



    /* Zero Initialized section */
    __sbss = ALIGN(4);

    .bss (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);

        *(.bss .bss.*);
        *(COMMON);

        . = ALIGN(4);
    } > SRAM0

    __ebss = ALIGN(4);



    /* Uninitialized section */
    __suninit = ALIGN(4);

    .uninit : ALIGN(4)
    {
        . = ALIGN(4);

        *(.uninit .uninit.*);

        . = ALIGN(4);
    } > SRAM0 AT > FLASH

    __euninit = ALIGN(4);

    /* RAM allocated functions */
    .ramfn : ALIGN(4)
    {
        __sramfn = ALIGN(4);

        *(.ramfn .ramfn.*);

    } > SRAM0 AT > FLASH

    __eramfn = ALIGN(4);
    __lramfn = LOADADDR(.ramfn);



    /* Discarded sections. */
    /DISCARD/ :
    {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}

/* Boot section checks 
ASSERT( SIZEOF(.boot) > 0, "Boot section must be present in the image" );
ASSERT( SIZEOF(.boot) == 64, "Boot section must contain all fields and be 64 bytes long" );
*/