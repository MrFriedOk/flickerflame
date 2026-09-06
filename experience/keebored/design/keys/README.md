### Socket
I need to find out more but any hall effect switch should just fit on a regular mechanical switch socket, minus the hole in the middle and the metal contacts.
Using measurements from GATERON docs, I made a footprint in KiCAD using NPTH circle pads with 1.7mm diameter. Do note, the docs give 0.05 tolerance, make sure this measurement is accurate. Worst case, you just glue the switches in place :3 . 
#### Notes
Using any given HE switch, not custom, there will not be enough space for an HS or any electrical components really (besides led of course), I have moved the HS, caps and resistors behind the key with just the led in the front. HS alignment is not affected, but is it ok to just put an HS behind the board? riskable's custom "void switches" solve this problem by making room for LED and the HS, the only issue being that I do not have access to a 3D printer.

--- 
### Owlab Ti HE
These switches (according to a few subreddits and product reviews) are well regarded for their sound, which is not as important to me as just having HE switches. 
Biggest issue with these is that I can't find documentation for them.
### GATERON KS-20 Magnetic White HE Switch Set
I was able to find documentation on these switches, very helpful for making a custom board, I need to know dimensions.