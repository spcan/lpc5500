    /* .got section is used to detect if its use */
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

