    /* CPU 0 Vector Table */
    .vt0 (NOLOAD) : ALIGN(512)
    {
        /* Align the section to 512 bytes */
        . = ORIGIN(STACK0);

        /* Export the VT0 start symbol */
        __svt0 = .;

        /* Keep the vector table */
        KEEP(*(.vtable.CORE0));

        /* Allocate 320 bytes to the Vector Table */
        . = ORIGIN(STACK0) + 320;

        /* Export the VT0 end symbol */
        __evt0 = .;
    } > STACK0

    /* CPU 0 Stack */
    .stack0 (NOLOAD) :
    {
        /* Set the start of the stack to the end of the Vector Table */
        . = ORIGIN(STACK0) + 320;

        /* Export the STACK0 start symbol */
        __sstack0 = .;

        /* Allocate to the end of the STACK 0 section */
        . = ORIGIN(STACK0) + LENGTH(STACK0);

        /* Export the STACK0 end symbol */
        __estack0 = . - 4;
    } > STACK0

