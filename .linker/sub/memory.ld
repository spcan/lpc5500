    /* CPU 1 Vector Table and Stack region. 16 kiB total. */
    /* TODO : Move this section to SRAMX */
    STACK1(rw) : ORIGIN = ORIGIN(STACK0) + LENGTH(STACK0), LENGTH = 8K

