# Switches
Because I am using just 4 keys, I can easily use analog for my keys. This provides a few benefits, mainly the fact that input is not a binary 1 or 0. With this, I can make the firmware do pretty much anything in terms of actuation, like send input at 1% actuation if for some reason I wanted to do that.
Hall effect switches do not have any pins, no method of communication. Their primary purpose is literally to move a magnet, I will need to place a hall sensor very accurately in the middle of the switch, right 
where the magnet would be to get good readings.

---
## Socket
I need to find out more but any hall effect switch should just fit on a regular mechanical switch socket, minus the hole in the middle and the metal contacts.
Using measurements from GATERON docs, I made a footprint in KiCAD with NPTH circle pads with 1.7mm diameter. Do note, the docs give 0.05 tolerance, make sure this measurement is accurate. Worst case, you just glue the switches in place :3 . 
### Notes
Using any given HE switch, not custom, there will not be enough space for an HS or any electrical components really (besides led of course), I have moved the HS, caps and resistors behind the key with just the led in the front. HS alignment is not affected, but is it ok to just put an HS behind the board? Riskable's 

## Owlab Ti HE
These switches (according to a few subreddits and product reviews) are well regarded for their sound, which is not as important to me as just having HE switches. 
Biggest issue with these is that I can't find documentation for them.


## GATERON KS-20 Magnetic White HE Switch Set
I was able to find documentation on these switches, very helpful for making a custom board, I need to know dimensions.