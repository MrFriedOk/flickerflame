# I2C
Referring specifically to the OLED display.
## Bus
On a single I2C bus, multiple devices can be connected, only one device can actually *use* the bus though (assuming a driver is aimed specifically for *one* device). 

> [!NOTE] So, can you share I2C?
> I confused myself a bit and need to make it clear.
> 
> Yes. Just like SPI chip select, I2C uses addresses and busses can be shared using a wrapper like those given by `embedded-hal-bus`. These wrappers let each driver communicate to a device on the (shared) bus as if it were an independent bus. 
> Wrappers 'lock' a bus temporarily, preventing communication until the driver (which is attached to an I2C address) finishes a task.
## Speed
RP2040 offers two speeds, 100kb/s or 400kb/s. 1mb/s is available as well but will not be needed.
### How much data transfer?
Each frame uses 1024 byes (128x64) and factoring in the *worst case* (30fps _Bad Apple!!_), this is about 260 kb/s. Can't use slow speed but 400kb/s is more than enough.
## Mode
[^1]The RP2040's I2C controllers support controller or slave mode, but both can't be used at the same time on each controllers. Refer to [mcu](../chips/mcu.md), the RP2040 has 2x I2C controllers (I2C0, I2C1)
**Master**
- The controller
- Will be used for display
**Slave**
- The target
- Will likely *not* be used at all
### Async or Nah?
I2C is synchronous, processes wait until *both* 'parties' are finished. Async allows one process to continue independently. 
## Sources Cited
[^1]: ImplFerris. rp2040-book. _github.com_ https://rp2040.implrust.com/i2c/rp2040-pico-i2c.html. Accessed October 6, 2026.
