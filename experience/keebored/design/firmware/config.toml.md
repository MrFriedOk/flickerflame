# Config
Configuring my package can greatly improve efficiency. For example, using `runner` to flash the board when building, saves lots of time.

Here is an example influenced by [the rp2040-hal examples](https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/.cargo/config.toml),

> [!NOTE] Runner
> Make sure to use `cargo run` to ensure the runner is used.


```toml
#
# Cargo Configuration for the https://github.com/rp-rs/rp-hal.git repository.
#
# You might want to make a similar file in your own repository if you are
# writing programs for Raspberry Pi microcontrollers.
#

[build]
# set target to match cortex
target = "thumbv6m-none-eabi"

# Target specific options, for the rp2040 architecture
[target.thumbv6m-none-eabi]
# pass options to rustc, tell linkers to link

# disable loop-vectorizer, cortex doesn't support simd. 
rustflags = [
    "-C", "no-vectorize-loops",
    "-C", "link-arg=-Tlink.x" #use link.x, make sure build.rs is there
]

# flash over usb,
runner = "picotool load --update --verify --execute -t elf"

# for swd probes,
# runner = "probe-rs run --chip RP2040"
```