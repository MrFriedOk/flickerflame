# Keebored (idk what to name it)
## Objective
My goal with this project is to familiarize myself with KiCAD and other CAD-ing tools so I can move towards building a mouse. Not only that, but I can *maybe* get a functional keyboard sort of thing to use to play osu! or something.
## Overview
I would like the keyboard to have LED, how many I do not know but let's shoot for one ARGB per key, yeah?
Overkill but I would like to use the RP2040, close MCU to the RP2350 (and its variants) that I plan to use for the mouse. This MCU has exactly enough ADC pins for my key layout.

## Key Layout
This is not going to be a large keyboard, \[**determine dimensions**] it will feature four keys in order from top to bottom and left to right, ESC, C, Z, X. These are literally the only keys I need for osu!

### Why Analog?
See [this page](./switches/README.md). Analog provides more flexibility in terms of actuation.

## Voltage Regulation
I decided to use a buck converter as opposed to a traditional linear voltage regulator. I wanted to keep power consumption as efficient as possible, and with a buck converter, you get proper voltage with a tradeoff of noise. 

The buck converter needs to do these things:
- Input 5V from USB
- Output 3V3
- Minimize noise

### TPS54302
This is the current buck converter I am using. A formula is given to determine appropriate resistor ohmage to get the desired voltage output.

The formula can be expressed like this, 
Voltage output is equal to voltage reference (the datasheet shows this as .596) multiplied by resistor 2 (100k is a recommended value) divided by resistor 3 plus one.