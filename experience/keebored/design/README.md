# Design

## Dimensions

I have outlined a couple of options to choose from and the determining factor in which one depends almost entirely on which display I want to use, the SSD1306 or SSD1309 (both their respective 128x64 variant), as they take up the most space and already allow for room to put the keys and rotary encoder down. Revision 3 was designed with the SSD1306 in mind, though I can make multiple designs if needed, and I likely will do. The most compact design that I have currently is revision 3, I do not want to make it smaller, as it is already likely small enough to comply with my decision to make the keyboard portable. In fact, revision 3 might be *too* small, potentially causing issues if there is not enough weight. Example, using the rotary encoder, and the board being so small that it shifts. This can be countered with weights, or by using a larger design. 
## Revision
Still technically iteration 1 but it is worth mentioning, I have revised the PCB once before (09/05/26) and I am going to do it again. 
### Issues

#### PCB Revision 1
The first revision, I basically took the template and removed stuff I didn't want, then added my own stuff. This ended up not working because I got long, ugly traces and really, the whole design was lazy. When I could have placed the MCU in a better spot relative to the hall sensors, I just routed without really thinking.

#### PCB Revision 2 09/05/26
This time, I had moved the MCU a bit in order to accommodate the hall sensors. It *would* have worked, only I still had the weird looking PCB. Routing was still a bit lazy, when I really should have 100% finalized the position of every component *before* routing. I put more thought into traces, "what will this block" or "is there enough room for GND" for example, but it could have been better

#### PCB Revision 3 (future)
I want to reroute everything again, this time having the MCU even *closer* to the hall sensors. I also want to fix my HE switch footprint, adding a spot for a new LED that would work better for this board, fitting *behind* the PCB instead of on the top. I would also like to remove the entire 3V3 layer, as I do not need it. With 4 keys and hardly any GPIO usage, it's borderline laziness if I just stuck with an entire layer of 3V3. What is the best placement for components if I want to go this *route*? Should I place the MCU closer to the bottom and what affect does this have on USB? Another major change is the fact that I am going to be using different LEDs, they require higher voltage, so I will need to implement a larger 5V/VBUS power rail into the design. I don't want to say that I am "doing this because they did it" but this revision was highly inspired by corne design choices.
 