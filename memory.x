MEMORY
{
    FLASH                    : ORIGIN = 0x00000000, LENGTH = 508K
    NSC_FLASH                : ORIGIN = 0x0007F000, LENGTH = 4K
    NS_FLASH                 : ORIGIN = 0x00080000, LENGTH = 512K
    
    RAM                      : ORIGIN = 0x20000000, LENGTH = 128K
    NS_RAM                   : ORIGIN = 0x20020000, LENGTH = 128K
}

_NS_VENEERS = ORIGIN(NS_FLASH);
_NSC_VENEERS = ORIGIN(NSC_FLASH);

_nsc_flash_start = ORIGIN(NSC_FLASH);
_nsc_flash_end = _nsc_flash_start + LENGTH(NSC_FLASH);

_ns_flash_start = ORIGIN(NS_FLASH);
_ns_flash_end = _ns_flash_start + LENGTH(NS_FLASH);
_ns_ram_start = ORIGIN(NS_RAM);
_ns_ram_end = _ns_ram_start + LENGTH(NS_RAM);

SECTIONS
{
  /* ### .ns_vectors */
  .nsc_vectors ORIGIN(NSC_FLASH) :
  {
    KEEP(*(.nsc_veneers.searcher));
    KEEP(*(.nsc_veneers));
    . = . + 12; /* Add an empty veneer at the end that should end up as 0's to indicate that we've reached the end */
    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
  } > NSC_FLASH
}
