# Power
### Important!!
Power rails must be clean enough to share (as an extreme, and from example used in the board) 3v3 with the MCU, an OLED breakout board and hall sensors. Hall sensors get top priority, ~~~***should I consider a separate*** LDO for hall sensing or is 10uF caps near each one enough?~~~ Do NOT use a separate LDO, voltage reference MUST be the same. Consider giving OLED 5v and rely on beefy caps **close** to *every* hall sensor.
## Overview
The board will consist of two power rails, +5V, given by [VBUS](usb.md) and +3V3 coming from the LDO. Two layer board, both planes being GND, plenty of room for ground.

## Voltage Regulation
**Step 5V VBUS down to 3V3**
I originally decided to use a buck converter as opposed to a traditional linear voltage regulator. I wanted to keep power consumption as efficient as possible, and with a buck converter, you get proper voltage with a tradeoff of noise. 
### NCP1117ST33T3G
SOT-223 LDO, 3V3 to 5V. This is the LDO I am currently using, and the one used in the RP2040 [example](https://pip-assets.raspberrypi.com/categories/814-rp2040/documents/RP-008279-DS-2-hardware-design-with-rp2040.pdf). 

> [!Design tip:]
>  I do not necessarily need 5V close to LDO, keep this in mind with LED.

### Buck
The buck converter would need to do these things:
- Input 5V from USB
- Output 3V3
- Minimize noise
I ultimately dropped the buck converter, as I was unsure if it would add too much noise for the analog keys, and there was no real benefit to using one over an LDO.
Buck converters also **require inductors**, something that I am not willing to risk having on my board. Yes, the noise isn't *that* much, but still a risk nonetheless.
### TPS54302
This is the buck converter I ~~~am~~~ was using. A formula is given to determine appropriate resistor ohmage to get the desired voltage output.
The formula can be expressed like this, 
Voltage output (V_O) is equal to voltage reference (V_r) (the datasheet shows this as .596) multiplied by resistor 2 (100k is a recommended value) divided by resistor 3 plus one. Wonderfully typed out.
$$
V_O=V_r((R2/R3)+1), somethig like that
$$
> [!NOTE] Note
> Hey, you do realize you can make math blocks, right? ///v///


