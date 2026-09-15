# Display
I would like to put a display on the negative space left by the shell being long enough to support a rotary encoder above the keys, and wide enough for the keys, as well as some padding because it looks better lol. I intend on a 128x64 OLED display
The display, as mentioned below, should not be attached directly to the board. I2C will still be used, but I should instead make holes, relative to the given device's dimensions and pinout on the board so I can instead use pogo pins or whatever works best to externally "mount" the display. This adds modularity, as well as convenience. 

## Verdict
I need to choose a display, or both and design multiple PCBs but either way, they must support I2C and must be 128x64. They must also be compatible with the fact that I will not be using direct-to-board display. Thankfully, these displays listed below are *everywhere*, often in breakout boards which is exactly what I am looking for.
### SSD1309
Available in 128x64 and 128x32 variants, this display is often much larger than the SSD1306, with the same resolution. Because the resolution will be low either way, I do not mind if pixel density is lower.
#### Pros/cons
Whether or not this is a good thing or bad thing depends on what I believe its impact on portability is, but with the display being larger, the PCB will also need to be larger. Having a huge display, like *four keys* wide is going to attract attention but maybe this is the goal, bad apple might get tiresome on though. Larger PCB forces me to use a different key layout, which is something that I have considered anyways so I can reach keys or rotary encoders easier. 
### SSD1306
Same as SSD1309, 128x64 and 128x32 are available, and is much smaller than the SSD1309.

## Notes
### OLED-128O064D-BPP3N00000
By Vishay, this OLED display is 128x64 resolution

May not use, as the connector would be wired directly to the board, adding clutter and inconvenience. I also do not need to use a specific footprint, I can instead use one that already has breakout boards or clones for. 

