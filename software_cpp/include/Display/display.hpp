#pragma once

#include <GFX.hpp>

#include "hardware/i2c.h"
// #include <logo.hpp>
#include <Config.h>
#include <FreeRTOS.h>
#include <custom-logo.h>

class Display {
 private:
  // Two ways to create a GFX object:
  // GFX *oled = new GFX(I2C_ADDR, DISPLAY_SIZE, I2C_PORT); // Stack Allocation
  GFX oled = GFX(I2C_ADDR, DISPLAY_SIZE, I2C_PORT);  // Heap Allocation

 public:
  Display();
  ~Display();

  void clear();
  void test();
  void showSplashScreen();
  // void drawString(uint8_t x, uint8_t y, const char *str);
  // void drawProgressBar(uint8_t x, uint8_t y, uint8_t width, uint8_t height,
  // uint8_t progress); void display();
};