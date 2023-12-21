/* Assert no dynamic relocations */
ASSERT( SIZEOF(.got) == 0,
    "ERROR(lpc5500-boot): .got section is not allowed. Dynamic relocations are not
allowed. If linking C code, modify the build without the -fPIC flag." );

