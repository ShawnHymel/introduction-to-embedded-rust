MEMORY {
    /* First 256 bytes is for the second stage bootloader */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100

    /* Rest of the 2 MB flash is for the program */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    /* 264 kB of on-chip SRAM, treat all 6 banks as one region */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

EXTERN(BOOT2_FIRMWARE)

/* Put .boot2 section (defined in the rp2040-boot2 crate) into the BOOT2 area of memory */
SECTIONS {
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
