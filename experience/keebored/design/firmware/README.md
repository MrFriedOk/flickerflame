# Firmware
The keyboard will be controlled by embedded code, I need to know how to write embedded code.

## Language
I have decided to use **Rust**. I also must decide on an edition. I do not know what factors would influence which edition I use. As far as I know, there isn't a downside to using the latest edition, crates are seemingly compatible with later editions, Rust aims for this philosophy. 
## Experience
**What can I do now**? I have many opportunities to learn. RP2040 devboard ensures I can get experience with the MCU I will use for the keyboard. Access to OLED teaches me how I will use it in the keyboard. I also use arch btw
## Requirements
What must the language be able to do:
- Low level
- Learnable by me 
- Have examples
- Known to work well with embedded
## Options
A few options that immediately come to mind are available:
- C
	- Low level, fast, tried and true
- Rust
	- Modern
	- Low level
	- Strict compile-time rules (efficient and safe)
	- Good docs + package ecosystem
	- 
- Python
	- No
- Assembly
	- ***Low*** level
	- Unecessary
	- No

> [!NOTE] Note,
> Idk why I made this

```mermaid
radar-beta
  title Weight Chart
  axis docs["Docs"], efficiency["Efficiency"], ease["Ease"]
  curve rust["Rust"]{80, 95, 50}
  curve python["Python"]{80, 40, 85}
  max 100
  min 0
```

[back](../../README.md)