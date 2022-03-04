#include <stdio.h>
#include <stdint.h>
#include "hardware/adc.h"
#include "pico/stdlib.h"

#include "hardware/i2c.h"
#include <logo.hpp>
#include <GFX.hpp>

int main()
{
  // Blink LED
  // const uint led_pin = 25;

  // gpio_init(led_pin);
  // gpio_set_dir(led_pin, GPIO_OUT);

  // while (true)
  // {
  //   gpio_put(led_pin, true);
  //   sleep_ms(1000);
  //   gpio_put(led_pin, false);
  //   sleep_ms(1000);
  // }

  stdio_init_all();
  i2c_init(i2c0, 400000);
  gpio_set_function(0, GPIO_FUNC_I2C); // Use GPIO2 as I2C
  gpio_set_function(1, GPIO_FUNC_I2C); // Use GPIO3 as I2C
  gpio_pull_up(0);                     // Pull up GPIO2
  gpio_pull_up(1);                     // Pull up GPIO3

  GFX oled(0x3C, size::W128xH32, i2c0);

  oled.display(logo); // Display logo

  while (true)
  {
    sleep_ms(1000);
    oled.clear(); // Clear buffer
    oled.drawString(0, 0, "Raspberry Pico");
    oled.drawString(0, 10, "Oled Example");
    oled.drawString(0, 20, "Have fun!");
    oled.drawProgressBar(0, oled.getHeight() - 5, oled.getWidth(), 5, rand() % 100 + 1);

    oled.display(); // Send buffer to the screen
  }
  return 0;
}
