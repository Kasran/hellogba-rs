OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)

ENTRY(__start)

/* the bios needs the last 256 bytes of iwram reserved for stuff */
__bios_reserved = 256;

MEMORY {
    ewram   : ORIGIN = 0x02000000, LENGTH = 256K
    iwram   : ORIGIN = 0x03000000, LENGTH = 32K - __bios_reserved
    rom     : ORIGIN = 0x08000000, LENGTH = 32M
}

SECTIONS {
    .entrypoint : {
        *(.entrypoint);
        . = ALIGN(4);
    } > rom

    .text : {
        *(.text .text*);
        . = ALIGN(4);
    } > rom

    .rodata : {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } > rom

    /* all the linker symbols are taken shamelessly from
     * agb's linker script */

    .iwram : {
        *(.iwram .iwram.*);
        . = ALIGN(4);

        *(.text_iwram .text_iwram.*);
        . = ALIGN(4);

    } > iwram AT>rom
    __iwram_data_start = ADDR(.iwram);
    __iwram_rom_start = LOADADDR(.iwram);
    __iwram_rom_length_halfwords = (SIZEOF(.iwram) + 1) / 2;
    __iwram_end = __iwram_data_start + SIZEOF(.iwram);

    .ewram : {
        *(.ewram .ewram.*);
        . = ALIGN(4);

        *(.data .data.*);
        . = ALIGN(4);

    } > ewram AT>rom
    __ewram_data_start = ADDR(.ewram);
    __ewram_rom_start = LOADADDR(.ewram);
    __ewram_rom_length_halfwords = (SIZEOF(.ewram) + 1) / 2;

    .bss : {
        *(.bss .bss.*);
        . = ALIGN(4);
         = ABSOLUTE(.);
    } > ewram

    __ewram_data_end = __ewram_data_start + SIZEOF(.ewram) + SIZEOF(.bss);

    .shstrtab : {
       *(.shstrtab)
    }

    /* discard anything else */
    /DISCARD/ : { *(*) }
}