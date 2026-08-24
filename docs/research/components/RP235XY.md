The RP2350 (and its variants) is the successor to the RP2040, offering "significant performance" over the original. 
## Key Features
These apply to all variants
- Dual core @155mhz
- 520KB SRAM
- Security features (elaborate)
- USB 1.1 controller and PHY, host and device support
- 16MB additional QSPI flash (storage) or PSRAM (volatile) storage via second chip-select

| Variant | Package | Package Size (mm) | Internal Flash Mem | GPIO Pins | Analogue Inputs | PWM Channels |
| ------- | ------- | ----------------- | ------------------ | --------- | --------------- | ------------ |
| RP2350A | QFN-60  | 7 x 7             | N/A                | 30        | 4               | 16           |
| RP2350B | QFN-80  | 10 x 10           | N/A                | 48        | 8               | 24           |
| RP2354A | QFN-60  | 7 x 7             | Stacked 2MB        | 30        | 4               | 16           |
| RP2354B | QFN-80  | 10 x 10           | Stacked 2MB        | 48        | 8               | 24           |

## Difference in Variants
RP2350 vs. RP2354 (A or B variant), the RP2354 has internal flash memory, potentially removing the need for an external chip to hold flash (unless  you want like 4k bad apple or something). the RP2350 does not have internal flash memory, this would be the ideal choice if i knew i could not fit everything in 2MB. Do i need more than 2MB?

RP235XA vs. RP235XB (0 or 4 variant), either MCU's A variant uses the 60QFN package. The B variant uses the 80QFN package. 80QFN offers 48 GPIO, 60QFN offers 30. How much GPIO do i need? 
## Flash Memory
Assuming i will be using the RD2354 variants, the chip has 2MB of flash memory to work with. As of now, i do not know how much i would need but i can say that 2MB is likely enough for what i am working with, but i do not want to make a decision for an MCU based purely on "it's better so i might as well".
## *Memory* Memory
520KB is consistent across all variants, 520KB is most definitely enough for the device. But this is a bold assumption, even though 520KB is plenty (i believe), i dont have an accurate way to determine if it truly is enough for the components. According to a few sources though, 520KB is far more than enough for a mouse with a polling rate of 8khz. 8khz is not a polling rate i have decided on but i do want it at least 1khz

## GPIO
Again, making assumptions, the B variants give me 48 GPIO which is already plenty but how much do i actually need? I shouldnt get a B variant based on fear that 30 GPIO isn't enough. How do the other components interact with the MCU and how much GPIO is needed?


