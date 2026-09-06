## Voltage Regulation
I originally decided to use a buck converter as opposed to a traditional linear voltage regulator. I wanted to keep power consumption as efficient as possible, and with a buck converter, you get proper voltage with a tradeoff of noise. 
**Design tip:** I do not necessarily need 5V close to LDO, keep this in mind with LED.

The buck converter would need to do these things:
- Input 5V from USB
- Output 3V3
- Minimize noise

I ultimately dropped the buck converter, as I was unsure if it would add too much noise for the analog keys, and there was no real benefit to using one over an LDO.

### NCP1117ST33T3G
SOT-223 LDO, 3V3 to 5V. This is the LDO I am currently using, and the one used in the RP2040 [example](https://pip-assets.raspberrypi.com/categories/814-rp2040/documents/RP-008279-DS-2-hardware-design-with-rp2040.pdf). 


--- 
### TPS54302
This is the buck converter I ~~~am~~~ was using. A formula is given to determine appropriate resistor ohmage to get the desired voltage output.

The formula can be expressed like this, 
Voltage output is equal to voltage reference (the datasheet shows this as .596) multiplied by resistor 2 (100k is a recommended value) divided by resistor 3 plus one. Wonderfully typed out.