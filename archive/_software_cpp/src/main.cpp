#include <stdio.h>
#include <stdint.h>
#include "hardware/adc.h"
#include "pico/stdlib.h"

#include <FreeRTOS.h>

#include <Config.h>
#include <display.hpp>

int main()
{
  stdio_init_all();
  i2c_init(I2C_PORT, DISPLAY_FREQ);
  gpio_set_function(I2C_PIN_SDA, GPIO_FUNC_I2C); // Use GPIO2 as I2C
  gpio_set_function(I2C_PIN_SCL, GPIO_FUNC_I2C); // Use GPIO3 as I2C
  gpio_pull_up(I2C_PIN_SDA);                     // Pull up GPIO2
  gpio_pull_up(I2C_PIN_SCL);                     // Pull up GPIO3

  Display display;

  return 0;
}
