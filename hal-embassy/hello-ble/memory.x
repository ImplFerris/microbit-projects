MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  MBR                               : ORIGIN = 0x00000000, LENGTH = 4K
  SOFTDEVICE                        : ORIGIN = 0x00001000, LENGTH = 112K
  FLASH                             : ORIGIN = 0x0001C000, LENGTH = 396K
  RAM                               : ORIGIN = 0x20003410, LENGTH = 117744
}
