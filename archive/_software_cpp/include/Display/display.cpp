#include <display.hpp>

Display::Display()
{
    // xTaskCreate(display_task, "display_task", configMINIMAL_STACK_SIZE, &oled, 1, NULL);
}

Display::~Display()
{
}

void Display::clear()
{
    printf("Test clear");
}

void Display::showSplashScreen()
{
    // Stack Allocation
    oled.display(customLogo);
    // Heap allocation
    // oled->display(customLogo);
    // this->oled.display(customLogo);

    printf("Test splashScreen");
}

void Display::test()
{
    GFX oled(I2C_ADDR, DISPLAY_SIZE, I2C_PORT);
    oled.display(customLogo); // Display logo

    while (true)
    {
        sleep_ms(1000);
        oled.clear(); // Clear buffer
        oled.drawString(0, 0, "Pi Deck Pico");
        oled.drawString(0, 10, "On GitHub soon!");
        oled.drawProgressBar(0, oled.getHeight() - 5, oled.getWidth(), 5, rand() % 100 + 1);

        oled.display(); // Send buffer to the screen
    }
}