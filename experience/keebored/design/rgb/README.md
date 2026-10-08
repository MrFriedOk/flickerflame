# RGB
I would like the keyboard to have RGB.
A switch may not give any clearance for an LED, how can I solve this? An example from [corne](https://github.com/foostan/crkbd), the LEDs are placed *on the back* of the PCB.  

## Priority
This is genuinely harming my design process, I *need* to find out what I want to use for RGB.
### Needs
### Previous Mistakes
I made the poor assumption that I needed reverse mount LEDs. This doesn't make any sense.. like at all. 
Not only do most switches have space for LEDs, I am *literally* tailoring the footprint of where my switches will go to a *single document*.![README](../keys/README.md#Socket) This is genuinely a good doc, it gives me everything I need and it selects switches I would use but what does this mean for the LED?
## The LED


## SK6812MINI-E
Used in corne, these LEDs have an SMD footprint that would better fit my keyboard, all I would need to do is place it under the PCB. Operating voltage is 3.7~5.5, meaning I not be able to use my 3V3 power, could route VBUS to the LED.

## SK9822-EC20


- Can handle up to 5V, 3V3 suitable
- Data and clock outputs for "daisy-chaining"
- Requires 2 GPIO