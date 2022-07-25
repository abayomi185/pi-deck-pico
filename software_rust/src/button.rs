use embedded_hal::digital::v2::InputPin;

use rp_pico::hal;
use rp_pico::hal::gpio::Interrupt::EdgeLow;

use usbd_hid::descriptor::{KeyboardReport, MediaKey, MediaKeyboardReport};
use usbd_hid::hid_class::HIDClass;

pub enum Button {
    One(hal::gpio::Pin<hal::gpio::bank0::Gpio26, hal::gpio::FloatingInput>),
    Two(hal::gpio::Pin<hal::gpio::bank0::Gpio27, hal::gpio::FloatingInput>),
    Three(hal::gpio::Pin<hal::gpio::bank0::Gpio28, hal::gpio::FloatingInput>),
    Four(hal::gpio::Pin<hal::gpio::bank0::Gpio4, hal::gpio::FloatingInput>),
    Five(hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::FloatingInput>),
    Six(hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::FloatingInput>),
}

impl Button {
    pub fn set_button_interrupt(&self) {
        match self {
            Button::One(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            Button::Two(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            Button::Three(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            Button::Four(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            Button::Five(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            Button::Six(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
        }
    }

    pub fn clear_button_interrupt(&mut self) {
        match self {
            Button::One(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
            Button::Two(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
            Button::Three(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
            Button::Four(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
            Button::Five(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
            Button::Six(ref mut gpio) => gpio.clear_interrupt(EdgeLow),
        }
    }

    pub fn is_low(&self) -> bool {
        match self {
            Button::One(gpio) => gpio.is_low().unwrap(),
            Button::Two(gpio) => gpio.is_low().unwrap(),
            Button::Three(gpio) => gpio.is_low().unwrap(),
            Button::Four(gpio) => gpio.is_low().unwrap(),
            Button::Five(gpio) => gpio.is_low().unwrap(),
            Button::Six(gpio) => gpio.is_low().unwrap(),
        }
    }

    // pub fn delay(&self) {
    //     let mut delay = timer_a.count_down();
    //     delay.start(150_u32.milliseconds());
    //     let _ = nb::block!(delay.wait());
    // }

    pub fn send_key(
        &self,
        hid: &HIDClass<'static, hal::usb::UsbBus>,
    ) -> Result<usize, usb_device::UsbError> {
        let media_play_report = MediaKeyboardReport {
            usage_id: MediaKey::Play as u16,
        };

        match self {
            Button::One(_) => hid.push_input(&media_play_report),
            Button::Two(_) => hid.push_input(&media_play_report),
            Button::Three(_) => hid.push_input(&media_play_report),
            Button::Four(_) => hid.push_input(&media_play_report),
            Button::Five(_) => hid.push_input(&media_play_report),
            Button::Six(_) => hid.push_input(&media_play_report),
        }
    }
}

// USB HID Usage Tables
// https://www.usb.org/sites/default/files/documents/hut1_12v1.pdf
// Page: 56 for Function Keys.
// E.g Keyboard F13 is 104:(0x68) - decimal:hex
