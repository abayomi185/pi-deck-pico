# Adapting the example in https://learn.adafruit.com/adafruit-oled-featherwing/python-usage
# to use with Raspberry Pi Pico and CircuitPython

import board
import busio
import displayio
import terminalio
import adafruit_displayio_ssd1306
from adafruit_display_text import label

# Pins for key_switches
KEY_A = board.GP2
KEY_B = board.GP3
KEY_C = board.GP4
KEY_D = board.GP26
KEY_E = board.GP27
KEY_F = board.GP28

i2c = busio.I2C(scl=board.GP6, sda=board.GP7)  # This RPi Pico way to call I2C

display_bus = displayio.I2CDisplay(i2c, device_address=0x3C)  # The address of my Board

display = adafruit_displayio_ssd1306.SSD1306(display_bus, width=128, height=32)
splash = displayio.Group(max_size=10)
display.show(splash)

color_bitmap = displayio.Bitmap(128, 32, 1)  # Full screen white
color_palette = displayio.Palette(1)
color_palette[0] = 0xFFFFFF  # White

bg_sprite = displayio.TileGrid(color_bitmap, pixel_shader=color_palette, x=0, y=0)
splash.append(bg_sprite)

# Draw a smaller inner rectangle
inner_bitmap = displayio.Bitmap(128, 32, 1)
inner_palette = displayio.Palette(1)
inner_palette[0] = 0x000000  # Black
inner_sprite = displayio.TileGrid(inner_bitmap, pixel_shader=inner_palette, x=5, y=4)
splash.append(inner_sprite)

# Draw a label
text = "Nicolau dos"
text_area = label.Label(terminalio.FONT, text=text, color=0xFFFF00, x=28, y=15)
splash.append(text_area)

text = "Brinquedos"
text_area = label.Label(terminalio.FONT, text=text, color=0xFFFF00, x=32, y=25)
splash.append(text_area)

while True:
    pass
