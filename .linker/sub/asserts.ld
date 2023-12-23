/* Assert the vector tables are aligned to 512 bytes */
ASSERT( __svt1 % 512 == 0,
    "BUG(lpc5500): Core 1 Vector Table is not 512 byte aligned");

/* Assert the Stack End is aligned to 4 bytes */
ASSERT( __estack1 % 4 == 0,
    "BUG(lpc5500): Core 1 Stack Pointer is not 4 byte aligned");

