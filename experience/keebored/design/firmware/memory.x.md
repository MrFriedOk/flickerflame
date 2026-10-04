# The `memory.x` File
Rust needs a map for linkers to know where to put stuff, let's figure out some stuff.
## ROM
- `0x00000000`
- Directly connected to RP2040 bus fabric 
- Bootloader RO, wont use it
- 16kB
## XIP
- `0x10000000`
- Will be the `FLASH` origin in `memory.x`
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
MEMORY {
FLASH : ORIGIN = 0x10000000, LENGTH = 16000K
RAM : ORIGIN = 0x20000000, LENGTH = 264K
}
```