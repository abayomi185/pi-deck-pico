#include <Arduino.h>

void setup()
{
    Serial.begin(9600);
    Serial.println("setup");

    // setup the pins
    pinMode(LED_BUILTIN, OUTPUT);
}

void loop()
{
    // blink the LED
    digitalWrite(LED_BUILTIN, HIGH);
    delay(500);
    digitalWrite(LED_BUILTIN, LOW);
    delay(500);
}