# The `memory.x` File
Rust needs a map for linkers to know where to put stuff, let's figure out some stuff.
## ROM
- `0x00000000`
- Directly connected to RP2040 bus fabric 
- Bootloader RO, wont use it
- 16kB
## BOOT2
- The RP2040 uses multi-stage bootloading, boot ROM accessed 256 bytes (0x100) from flash mem. 
## XIP
- `0x10000000`
- `FLASH` origin in `memory.x`, will be offset 0x100
- XIP connected to QSPI, connected to 16MB flash
## SRAM
- `0x20000000`
- `RAM` origin
- Directly connected to RP2040 bus fabric
- 264kB, divided into 6 partitions
	- Unimportant, most software treats as a single region
## Bring it Together
Now that I have this all sorted out, I can write it out.
```
/*
specify secondary boot, 256 bytes
specify flash offset 256 bytes
specify ram
*/
MEMORY {
BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
FLASH : ORIGIN = 0x10000100, LENGTH = 15625K - 0x100

/*
256kB is first 4 large banks, i can separate 0x20040000 and 0x20041000
if i need to, figure out if that is needed
for now, single memory region is fine
*/
RAM : ORIGIN = 0x20000000, LENGTH = 264K
}
```