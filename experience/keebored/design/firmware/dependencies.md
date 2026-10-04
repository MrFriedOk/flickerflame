# Dependencies
What to include in the [`Cargo.toml`](../../keebored/firmware/Cargo.toml)
- [`rp2040-hal`](https://docs.rs/rp2040-hal/latest/rp2040_hal/), version 0.12.0, HAL (hardware abstraction layer) for RP2040. 
- `embedded-hal`, version 1.0.0, HAL for embedded systems
- `image`, will not be used on the board, good for converting images on a PC ![badApple](badApple.md#Crates)
- `cortex-m`, version 0.7.9, low level access to cortex-m processors.
	- ![memory.x](memory.x.md), reference addresses for linkers
- `ssd1306`, version 0.10.0, ssd1306 display driver
	- requires, `embedded-graphics`, another thing that will be used
- `panic-halt`, version 1.0.0, sets panic mode to halt (not sure what the benefit of this over others is)
- `panic-semihosting`, version 0.7.0, useful for debugging (if I can get some debugging stuff)
	- requires `cortex-m-semihosting`
	- see `panic-probe` as well, offers similar debugging with `probe-run`
- `ssd1309`, version 0.4.0, already contains most dependencies by `ssd1306`, but it's an 'official' driver for ssd1309. 
	- I can use this instead, even if ssd1306 drivers 'just work'
- `rp2040-boot2`, version 0.3.0, ROM requires 256 bytes of secondary boot
	- `pub static BOOT_LOADER_W25Q080: [u8; 256]`, bootloader to use for me
