
from PIL import Image
import os

for filename in sorted(os.listdir('.')):
    if filename.startswith('bw_frame_') and filename.endswith('.png'):
        img = Image.open(filename)
        img = img.convert('1')  # 1-bit mode
        
        # Convert to byte array (8 pixels per byte, vertical byte packing for SSD1306)
        width, height = img.size  # should be 128x64
        bytes_per_row = width // 8
        data = []
        
        for y in range(0, height, 8):
            for x in range(width):
                byte = 0
                for bit in range(8):
                    if y + bit < height:
                        pixel = img.getpixel((x, y + bit))
                        if pixel:  # white pixel
                            byte |= (1 << bit)
                data.append(byte)
        
        # Write to .h file
        output_file = filename.replace('.png', '.h').replace('bw_', 'frame_')
        with open(output_file, 'w') as f:
            f.write(f"const unsigned char {output_file[:-2]}[] PROGMEM = {{\n")
            for i, b in enumerate(data):
                f.write(f"0x{b:02x}")
                if i < len(data) - 1:
                    f.write(", ")
                if (i + 1) % 16 == 0:
                    f.write("\n")
            f.write("\n};\n")
