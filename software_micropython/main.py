# from machine import Pin, I2C
# from ssd1306 import SSD1306_I2C

# def main():
#     print("Welcome to RT-Thread MicroPython!")

# if __name__ == '__main__':
#     main()

from machine import Pin
import time

pin = Pin(25, Pin.OUT)

while True:
    pin.toggle()
    time.sleep_ms(1000)
