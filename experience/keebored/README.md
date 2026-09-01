## Voltage Regulation
I decided to use a buck converter as opposed to a traditional linear voltage regulator. I wanted to keep power consumption as efficient as possible, and with a buck converter, you get proper voltage with a tradeoff of noise. 

The buck converter needs to do these things:
- Input 5V from USB
- Output 3V3
- Minimize noise

### TPS54302
This is the current buck converter I am using. A formula is given to determine appropriate resistor ohmage to get the desired voltage output.

The formula can be expressed like this, 
Voltage output is equal to voltage reference (the datasheet shows this as .596)