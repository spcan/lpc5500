/* Assert the vector tables are aligned to 320 bytes */
ASSERT( __svt0 % 320 == 0,
    "BUG(lpc5500): Core 0 Vector Table is not 320 byte aligned");

/* Assert the Stack End is aligned to 4 bytes */
ASSERT( __estack0 % 4 == 0,
    "BUG(lpc5500): Core 0 Stack Pointer is not 4 byte aligned");

