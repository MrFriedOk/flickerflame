## Component Dump
Here is where I will put a bunch of ideas i have for components to use
- [RP235X](../research/components/RP235XY.md)(0/4, A/B, what is the difference?)
- ESP32
- nRF52840

## 8khz
Because funny, .125ms polling rate. Would require USB 2.0+, whether or not it would mean including a USB controller or not doesn't matter.
## Wireless
I would like the mouse to be able to use 2.4ghz wireless. I would need to design a receiver, and I would need to figure out how to implement that in a custom design.

**Notes**
May influence MCU choice, RP235X for example would need external radio.


## Panels
I like having stuff on the side of my mouse, it'd be interesting if I could design some panels to be used.
"Smart" panels would not have an MCU, this is way overkill. They would simply act as an extension to the main board with the same power rails, and every panel would need to have the same pinout, as the main board determines what pins are used. 
**Questions**
- Will analog be used (probably yes in some way)
- Can I "compress" data so less pins can be used if I had many keys or components with many pins. How do data pins get what is needed across the board without too much pins used
- Will 5V realistically be needed. Probably no, most OLEDs, especially breakout boards if used can function with 3V3 and uber bright LEDs will not only be excessive, but annoying
- What kind of connection is realistic, strong and (honestly) aesthetically pleasing.
## Pin assessment
How many pins will be used? influence on MCU choice
### Panel Pins?
What data and power needs to reach each panel? Can I keep it universal for every panel?
I need to figure out how I want to get data across the panel and main board

| Net  | Functions | Notes                                                                   |
| ---- | --------- | ----------------------------------------------------------------------- |
| +5V  | Power     | 5V may be necessary if OLED or strong LED will be used, but not certain |
| +3V3 | Power     | Must match logic on the board, cant step 5V down for example            |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
|      |           |                                                                         |
