/* Assert .data is 8 byte aligned */
ASSERT( (__sdata % 8 == 0) && (__edata % 8 == 0),
    "BUG(lpc5500): .data must be 8-byte aligned");

/* Asset .data LMA is 4 byte aligned */
ASSERT( __sidata % 4 == 0,
    "BUG(lpc5500): .data LMA must be 4 byte aligned");

/* Assert .bss is 8 byte aligned */
ASSERT( (__sbss % 8 == 0) && (__ebss % 8 == 0),
    "BUG(lpc5500): .bss must be 8-byte aligned");

/* Assert .uninit is 8 byte aligned */
ASSERT( (__suninit % 8 == 0) && (__euninit % 8 == 0),
    "BUG(lpc5500): .uninit must be 8-byte aligned");

