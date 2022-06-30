from machine import Pin, I2C
from ssd1306 import SSD1306_I2C
import time

# Set up the I2C interface.
i2c = I2C(0, sda=Pin(0), scl=Pin(1), freq=400000)
# Set up the display.
display = SSD1306_I2C(128, 32, i2c)


def main():
    pin = Pin(25, Pin.OUT)

    # while True:
    #     pin.toggle()
    #     time.sleep_ms(1000)

    display.text("Hello, New World!", 0, 0, 1)
    display.show()


if __name__ == "__main__":
    main()
