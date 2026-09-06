## Voltage Regulation
I originally decided to use a buck converter as opposed to a traditional linear voltage regulator. I wanted to keep power consumption as efficient as possible, and with a buck converter, you get proper voltage with a tradeoff of noise. 

The buck converter would need to do these things:
- Input 5V from USB
- Output 3V3
- Minimize noise

I ultimately dropped the buck converter, as I was unsure if it would add too much noise for the analog keys

### TPS54302
This is the buck converter I ~~~am~~~ was using. A formula is given to determine appropriate resistor ohmage to get the desired voltage output.

The formula can be expressed like this, 
Voltage output is equal to voltage reference (the datasheet shows this as .596) multiplied by resistor 2 (100k is a recommended value) divided by resistor 3 plus one. Wonderfully typed out.