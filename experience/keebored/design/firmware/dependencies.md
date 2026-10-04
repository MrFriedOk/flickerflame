# Dependencies
What to include in the [`Cargo.toml`](../../keebored/firmware/Cargo.toml)
- [`rp2040-hal`](https://docs.rs/rp2040-hal/latest/rp2040_hal/), version 0.12.0, HAL (hardware abstraction layer) for RP2040. 
- `image`, ![badApple](badApple.md#Crates)
- `cortex-m`, version 0.7.9, low level access to cortex-m processors.
- `ssd1306`, version 0.10.0, ssd1306 display driver
	- requires, `embedded-graphics`, another thing that will be used
- `panic-halt`, version 1.0.0, sets panic mode to halt (not sure what the benefit of this over others is)
- `panic-semihosting`, version 0.7.0, useful for debugging (if I can get some debugging stuff)
	- requires `cortex-m-semihosting`
	- see `panic-probe` as well, offers simialr debugging with `probe-run`
