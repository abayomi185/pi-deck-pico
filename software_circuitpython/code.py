# Adapting the example in https://learn.adafruit.com/adafruit-oled-featherwing/python-usage
# to use with Raspberry Pi Pico and CircuitPython

import time
import json
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

# getattr() function to get the value of a variable from a string
# print("Waiting for key pin...")


class ConfigHelper:
    def __new__(cls):
        if not hasattr(cls, "instance"):
            cls.instance = super(ConfigHelper, cls).__new__(cls)
        return cls.instance

    def __init__(self):
        self.config = self.load_config()

    def load_config(self):
        with open("conf.json") as config_file:
            config = json.load(config_file)
        print("Config loaded")
        return config


class DeckDisplay:
    def __init__(self):
        self.load_display_config()
        self.initialise_display()
        self.show_text_banner()

    def load_display_config(self):
        self.display_config = ConfigHelper().config["display"]

    def initialise_display(self):
        scl_pin = getattr(board, self.display_config["scl"])
        sda_pin = getattr(board, self.display_config["sda"])
        display_address = int(self.display_config["address"], 16)
        display_width = int(self.display_config["width"])
        display_height = int(self.display_config["height"])

        displayio.release_displays()
        i2c = busio.I2C(scl=scl_pin, sda=sda_pin)
        display_bus = displayio.I2CDisplay(i2c, device_address=display_address)
        display = adafruit_displayio_ssd1306.SSD1306(
            display_bus, width=display_width, height=display_height
        )
        self.splash = displayio.Group()
        display.show(self.splash)

    def show_text_banner(self, text="Tech Tips!"):
        color_bitmap = displayio.Bitmap(128, 32, 1)
        color_palette = displayio.Palette(1)
        color_palette[0] = 0xFFFFFF  # White

        bg_sprite = displayio.TileGrid(
            color_bitmap, pixel_shader=color_palette, x=0, y=0
        )
        self.splash.append(bg_sprite)

        # Draw a smaller inner rectangle
        inner_bitmap = displayio.Bitmap(118, 24, 1)
        inner_palette = displayio.Palette(1)
        inner_palette[0] = 0x000000  # Black
        inner_sprite = displayio.TileGrid(
            inner_bitmap, pixel_shader=inner_palette, x=5, y=4
        )
        self.splash.append(inner_sprite)

        # Draw a label
        # text = "Tech Tips!"
        text_area = label.Label(terminalio.FONT, text=text, color=0xFFFF00, x=35, y=15)
        self.splash.append(text_area)


class DeckKeypad:
    def __init__(self):
        pass

    def load_keypad_config(self):
        self.key_config = ConfigHelper().config["keypad"]

    keypress_pins = [
        board.GP26,
        board.GP27,
        board.GP28,
        board.GP4,
        board.GP3,
        board.GP2,
    ]
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


def main():
    DeckDisplay()


if __name__ == "__main__":
    # deck_display()
    # deck_keyboard()
    # getattr(DeckDisplay, "initialise_display")()

    main()


# import microcontroller
# microcontroller
# import supervisor
# supervisor.reload()
