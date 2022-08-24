MEMORY
{
    FLASH                    : ORIGIN = 0x00000000, LENGTH = 512K
    NSFLASH                  : ORIGIN = 0x00080000, LENGTH = 512K
    RAM                      : ORIGIN = 0x20000000, LENGTH = 128K
    NSRAM                    : ORIGIN = 0x20020000, LENGTH = 128K
}

PROVIDE(_snstext = ORIGIN(NSFLASH));

SECTIONS
{
    .text.ns _snstext : {
        __snstext = .;

        *libnon_secure.a:*(.text .text.*)

        __enstext = .;
    } > NSFLASH
}
