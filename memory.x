MEMORY
{
    FLASH                    : ORIGIN = 0x00000000, LENGTH = 448K
    NSC_FLASH                : ORIGIN = 0x00070000, LENGTH = 64K
    NS_FLASH                 : ORIGIN = 0x00080000, LENGTH = 512K
    
    RAM                      : ORIGIN = 0x20000000, LENGTH = 128K
    NSC_RAM                  : ORIGIN = 0x20020000, LENGTH = 64K
    NS_RAM                   : ORIGIN = 0x20030000, LENGTH = 64K
}

_NS_VECTORS = ORIGIN(NS_FLASH);
_NSC_VECTORS = ORIGIN(NSC_FLASH);

SECTIONS
{
  /* ### .ns_vectors */
  .nsc_vectors ORIGIN(NSC_FLASH) :
  {
    KEEP(*(.nsc_vectors));
    . = . + 8; /* Add a vector at the end that should end up as 0's to indicate that we've reached the end */
    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
  } > NSC_FLASH

  /* ### .text */
  .nsc_text :
  {
    KEEP(*(.nsc_text.nsc_exported));
    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
  } > NSC_FLASH
}
