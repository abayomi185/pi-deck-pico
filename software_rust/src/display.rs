use rp_pico::hal;
use ssd1306::{prelude::*, Ssd1306};
type DisplayI2C = hal::I2C<
    hal::pac::I2C0,
    (
        hal::gpio::Pin<hal::gpio::bank0::Gpio0, hal::gpio::Function<hal::gpio::I2C>>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio1, hal::gpio::Function<hal::gpio::I2C>>,
    ),
>;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

pub fn show_text(
    display: &mut Ssd1306<
        I2CInterface<DisplayI2C>,
        DisplaySize128x32,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x32>,
    >,
    custom_text: &str,
) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(custom_text, Point::zero(), text_style, Baseline::Top)
        .draw(display)
        .unwrap();

    display.flush().unwrap();
}

// pub fn
