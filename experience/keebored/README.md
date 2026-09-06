# Keebored (idk what to name it)
## Objective
My goal with this project is to familiarize myself with KiCAD and other CAD-ing tools so I can move towards building a mouse. Not only that, but I can *maybe* get a functional keyboard sort of thing to use to play osu! or something.
## Overview
A small, semi-portable USB C keyboard.
### Key Features
- 128x64 OLED display via I2C
- Analog HE switches, directly to MCU ADC
- RGB
- ~55mm wide, ~95mm tall

I would like the keyboard to have LED, how many I do not know but let's shoot for one ARGB per key, yeah?
Overkill but I would like to use the RP2040, close MCU to the RP2350 (and its variants) that I plan to use for the mouse. This MCU has exactly enough ADC pins for my key layout.
The keyboard will also feature a rotary encoder or similar for volume control.
What better to do with negative space left from the rotary encoder than to put a good OLED display on the keyboard? But really, [this](./oled/README.md) is a real thing to consider, it teaches me I2C, something I will most definitely use in the future. 
## Key Layout
This is not going to be a large keyboard, \[**determine dimensions**] it will feature four keys in order from top to bottom and left to right, ESC, C, Z, X. These are literally the only keys I need for osu!

### Why Analog?
See [this page](./switches/README.md). Analog provides more flexibility in terms of actuation.

## Size
In reality, even without knowing exact dimensions, the keys alone add an absurd amount of space relative to all of the components. There is going to be a *lot* of negative space. What can I do with it?
## Index
[power](./power/README.md) 
