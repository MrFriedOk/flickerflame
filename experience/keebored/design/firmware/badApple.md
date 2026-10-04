# Bad Apple

> [!NOTE] Notice
> **This part of the code will not be used in embedded, I should probably move this**

I need to figure out how (using Rust) to convert a series of pngs I have, which were converted from an mp4 using ffmpeg, to make them grayscale and convert them to a format that can be given to the SSD1306.
> [!NOTE] SSD1309
> SSD1309 can be driven using the [same drivers as the SSD1306](../oled/display/README.md). 
## Crates
What crates will be used, outline some that may work.
- [`image`](https://docs.rs/image/latest/image/), version 0.25.10 provides image decoding, encoding and manipulation. 
	- **Notable type aliases, functions, traits, structs, modules
		- `Luma`, grayscale pixel
		- 
		- `open`, opens specified file given path, format determined **by file extension**
		- `GrayImage`, sendable grayscale image buffer
		- `image_dimensions`, can be done before loading an image (and is faster) to read a tuple containing width and height
		- `save_buffer`, saves supplied buffer to file at given path
		- `save_buffer_with_format`, saves supplied buffer to file given path