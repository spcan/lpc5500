    /* CPU 0 Vector Table */
    .vt0 (NOLOAD) : ALIGN(320)
    {
        /* Align the section to 320 bytes */
        . = ALIGN(320);

        /* Export the VT0 start symbol */
        __svt0 = .;

        /* Allocate 320 bytes to the Vector Table */
        . += 320;

        /* Export the VT0 end symbol */
        __evt0 = .;
    } > STACK0

    /* CPU 0 Stack */
    .stack0 (NOLOAD) :
    {
        /* Set the start of the stack to the end of the Vector Table */
        . = ADDR(.vt0) + SIZEOF(.vt0);

        /* Export the STACK0 start symbol */
        __sstack0 = .;

        /* Allocate to the end of the STACK 0 section */
        . = ORIGIN(STACK0) + LENGTH(STACK0);

        /* Export the STACK0 end symbol */
        __estack0 = . - 4;
    } > STACK0

