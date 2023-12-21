    /* 640 kiB FLASH region. Last 10 kiB of pages are reserved */
    FLASH(rx) : ORIGIN(0x00000000), LENGTH = 630K

    /* 64 kB ROM region */
    ROM(rx) : ORIGIN(0x03000000), LENGTH = 64K

