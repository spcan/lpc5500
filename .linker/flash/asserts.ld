/* Assert the size of the vector table is 16 entries * 4 bytes per entry (64 bytes) */
ASSERT( SIZEOF(.vector_table) == 64,
    "BUG(lpc5500): Size of vector table section is not 64 bytes");

/* Assert the the vector table is aligned to 320 bytes*/
ASSERT( __svt % 320 == 0,
    "BUG(lpc5500): .vector_table is not aligned to 320 bytes");

/* Assert FLASH is 4 byte aligned */
ASSERT( ORIGIN(FLASH) % 4 == 0,
    "BUG(lpc5500): Flash must be 4 byte aligned");

