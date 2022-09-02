MEMORY
{
    FLASH                    : ORIGIN = 0x00000000, LENGTH = 448K
    NSC_FLASH                : ORIGIN = 0x00070000, LENGTH = 64K
    NS_FLASH                 : ORIGIN = 0x00080000, LENGTH = 512K
    
    RAM                      : ORIGIN = 0x20000000, LENGTH = 128K
    NSC_RAM                  : ORIGIN = 0x20020000, LENGTH = 64K
    NS_RAM                   : ORIGIN = 0x20030000, LENGTH = 64K
}

_NS_CTOR_INIT = ORIGIN(NS_FLASH);
_NS_VT_LEN = ORIGIN(NS_RAM);
_NS_VT = _NS_VT_LEN + 4;
