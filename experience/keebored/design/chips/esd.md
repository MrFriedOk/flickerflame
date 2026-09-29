# ESD
Inspired by my incompetent decision to use a Schottky diode for ESD protection, I will outline ESD protection methods and why one would use them, as well as what to use in *my* case. 
## What's Left
I need to put ESD protection on VBUS, it is a core component of the board, of course. 

---
## What's Done
I already have--diagrammed in the schematic--USD_D+ and USB_D- nets ESD protected via a chip, TPD4E05U06DQA. By TI, it is a variant of [TPDxE05U06](https://www.ti.com/lit/ds/symlink/tpd4e05u06.pdf) and is used for high-speed signal lines built for ESD protection "above the IEC-61000-4-2 international standard". 
### Why
This chip was used because the DQA package supports "straight-through" routing, and fits exactly the amount of pins I need for USB ESD protection. Straight-through ensures that every line is ESD protected and avoids unnecessary amounts of diodes all over the place. 
## Schottky Problem
The original schematics used a Schottky diode between VBUS and what I called +5V (VBUS labeled *after* ESD protection) and because I was careless and didn't research, I had no idea what its purpose even was.
### What They Do
Yes, Schottky diodes *can* be used for ESD, just not in the way I used them. Traditionally, two diodes are placed from the signal line to VCC (to direct ESD to power) and another from GND to the signal line. The issue is the reliance on bulk decoupling caps near the power rail to absorb the spike, I don't want to rely on the assumption that the ones I have can keep the power rail stable enough.
## Use of TVS Diodes
TVS diodes are most appropriate for the VBUS line. [^1]TVS diodes act as "transparent" to the circuit and provides a low-impedence path for ESD directly to ground, clamping the transient voltage. Bidirectional can be used, but are most common for AC applications and isn't necessary. Unidirectional is what would be used.


> [!NOTE] Hey, me! :3
> Remember the TI TPDxE05U06? Yeah, remebmer how it has different formats, including an industry-standard SOD-523 package? It supports one channel, just use that one

---
[back](./README.md)

[^1]: Li, A. (June, 2026). ESD Protection for PCB Design: TVS, Schottky Clamps, and Layout Best Practices. *NextPCB*. https://www.nextpcb.com/blog/esd-protection-pcb-design-guide. Accessed September 29, 2026.
