# Adapting the example in https://learn.adafruit.com/adafruit-oled-featherwing/python-usage
# to use with Raspberry Pi Pico and CircuitPython

import time
import board
import busio
import digitalio
import displayio
import terminalio
import adafruit_ssd1306
import adafruit_displayio_ssd1306
from adafruit_display_text import label
import usb_hid
from adafruit_hid.keyboard import Keyboard
from adafruit_hid.keyboard_layout_us import KeyboardLayoutUS
from adafruit_hid.keycode import Keycode

displayio.release_displays()
i2c = busio.I2C(scl=board.GP1, sda=board.GP0)
display_bus = displayio.I2CDisplay(i2c, device_address=0x3C)
display = adafruit_displayio_ssd1306.SSD1306(display_bus, width=128, height=32)

# getattr() function to get the value of a variable from a string

keypress_pins = [board.GP26, board.GP27, board.GP28, board.GP4, board.GP3, board.GP2]
key_pin_array = []
keys_pressed = [
    Keycode.KEYPAD_ONE,
    Keycode.KEYPAD_TWO,
    Keycode.KEYPAD_THREE,
    Keycode.KEYPAD_FOUR,
    Keycode.KEYPAD_FIVE,
    Keycode.KEYPAD_SIX,
]
control_key = Keycode.SHIFT
time.sleep(1)
keyboard = Keyboard(usb_hid.devices)
keyboard_layout = KeyboardLayoutUS(keyboard)

for pin in keypress_pins:
    key_pin = digitalio.DigitalInOut(pin)
    key_pin.direction = digitalio.Direction.INPUT
    key_pin.pull = digitalio.Pull.UP
    key_pin_array.append(key_pin)

led = digitalio.DigitalInOut(board.LED)
led.direction = digitalio.Direction.OUTPUT

print("Waiting for key pin...")


def deck_display():
    # Make the display context
    splash = displayio.Group()
    display.show(splash)

    color_bitmap = displayio.Bitmap(128, 32, 1)
    color_palette = displayio.Palette(1)
    color_palette[0] = 0xFFFFFF  # White

    bg_sprite = displayio.TileGrid(color_bitmap, pixel_shader=color_palette, x=0, y=0)
    splash.append(bg_sprite)

    # Draw a smaller inner rectangle
    inner_bitmap = displayio.Bitmap(118, 24, 1)
    inner_palette = displayio.Palette(1)
    inner_palette[0] = 0x000000  # Black
    inner_sprite = displayio.TileGrid(
        inner_bitmap, pixel_shader=inner_palette, x=5, y=4
    )
    splash.append(inner_sprite)

    # Draw a label
    text = "Tech Tips!"
    text_area = label.Label(terminalio.FONT, text=text, color=0xFFFF00, x=35, y=15)
    splash.append(text_area)

    # while True:
    #     pass


def deck_keyboard():
    while True:
        # Check each pin
        for key_pin in key_pin_array:
            if not key_pin.value:  # Is it grounded?
                i = key_pin_array.index(key_pin)
                print(f"Pin {i} is grounded")

                # Turn on the red LED
                led.value = True

                while not key_pin.value:
                    pass  # Wait for it to be ungrounded!
                # "Type" the Keycode or string
                key = keys_pressed[i]  # Get the corresponding Keycode or string
                if isinstance(key, str):  # If it's a string...
                    keyboard_layout.write(key)  # ...Print the string
                else:  # If it's not a string...
                    keyboard.press(key)  # "Press"...
                    # keyboard.press(Keycode.KEYPAD_ONE)
                    keyboard.release_all()  # ..."Release"!

                # Turn off the red LED
                led.value = False

        time.sleep(0.01)


if __name__ == "__main__":
    deck_display()
    deck_keyboard()

# import microcontroller
# microcontroller

# import supervisor
# supervisor.reload()
