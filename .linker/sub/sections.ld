    /* CPU 1 Vector Table */
    .vt1 (NOLOAD) : ALIGN(512)
    {
        /* Align the section to 512 bytes */
        . = ORIGIN(STACK1);

        /* Export the VT1 start symbol */
        __svt1 = .;

        /* Keep the vector table */
        KEEP(*(.vtable.CORE1));

        /* Allocate 320 bytes to the Vector Table */
        . = ORIGIN(STACK1) + 320;

        /* Export the VT1 end symbol */
        __evt1 = .;
    } > STACK1

    /* CPU 1 Stack */
    .stack1 (NOLOAD) :
    {
        /* Set the start of the stack to the end of the Vector Table */
        . = ORIGIN(STACK1) + 320;

        /* Export the start symbol */
        __sstack1 = .;

        /* Allocate to the end of the STACK 1 section */
        . = ORIGIN(STACK1) + LENGTH(STACK1);

        /* Export the end symbol */
        __estack1 = . - 4;
    } > STACK1

