use embedded_hal::digital::v2::InputPin;

use rp_pico::hal;
use rp_pico::hal::gpio::Interrupt::{EdgeHigh, EdgeLow};

use usbd_hid::descriptor::{KeyboardReport, MediaKey, MediaKeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;
use usbd_hid::UsbError;

use crate::debouncer::Debouncer;

pub struct Button {
    pub variant: ButtonVariant,
    debouncer: Debouncer,
    pub is_pressed: bool,
}

impl Button {
    pub fn new(variant: ButtonVariant) -> Self {
        Self {
            variant,
            debouncer: Debouncer::new(5_000),
            is_pressed: false,
        }
    }

    pub fn debounce(&mut self, timer: &hal::timer::Timer, current_state: bool) {
        let current_time = timer.get_counter_low();
        self.debouncer.update(current_time, current_state);
        self.is_pressed = self.debouncer.stabilised_state;
    }
}

// Trait example for future reference
// trait ButtonDo {
//     fn is_pressed(&self) -> bool;
// }

// impl ButtonDo for Button {
//     fn is_pressed(&self) -> bool {
//         self.is_pressed()
//     }
// }

pub enum ButtonVariant {
    One(hal::gpio::Pin<hal::gpio::bank0::Gpio26, hal::gpio::FloatingInput>),
    Two(hal::gpio::Pin<hal::gpio::bank0::Gpio27, hal::gpio::FloatingInput>),
    Three(hal::gpio::Pin<hal::gpio::bank0::Gpio28, hal::gpio::FloatingInput>),
    Four(hal::gpio::Pin<hal::gpio::bank0::Gpio4, hal::gpio::FloatingInput>),
    Five(hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::FloatingInput>),
    Six(hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::FloatingInput>),
}

impl ButtonVariant {
    // TODO: I can use traits so I don't have to create and redeclare the gpio attr.
    // Would've saved a lot of time.

    pub fn set_button_interrupt(&self) {
        match self {
            ButtonVariant::One(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            ButtonVariant::Two(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            ButtonVariant::Three(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            ButtonVariant::Four(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            ButtonVariant::Five(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
            ButtonVariant::Six(gpio) => gpio.set_interrupt_enabled(EdgeLow, true),
        }

        // match self {
        //     ButtonVariant::One(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        //     ButtonVariant::Two(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        //     ButtonVariant::Three(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        //     ButtonVariant::Four(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        //     ButtonVariant::Five(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        //     ButtonVariant::Six(gpio) => gpio.set_interrupt_enabled(EdgeHigh, true),
        // }
    }

    pub fn clear_button_interrupt(&mut self) {
        match self {
            ButtonVariant::One(gpio) => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Two(gpio) => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Three(gpio) => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Four(gpio) => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Five(gpio) => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Six(gpio) => gpio.clear_interrupt(EdgeLow),
        }
    }

    // pub fn delay(&self) {
    //     let mut delay = timer_a.count_down();
    //     delay.start(150_u32.milliseconds());
    //     let _ = nb::block!(delay.wait());
    // }

    pub fn send_key(&self, hid: &HIDClass<'static, hal::usb::UsbBus>) -> Result<usize, UsbError> {
        let play_pause_report = MediaKeyboardReport {
            usage_id: MediaKey::Play as u16,
        };

        let volume_up_report = MediaKeyboardReport {
            usage_id: MediaKey::VolumeIncrement as u16,
        };

        let volume_down_report = MediaKeyboardReport {
            usage_id: MediaKey::VolumeDecrement as u16,
        };

        let keyboard_report = KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0x38, 0, 0, 0, 0, 0],
        };

        match self {
            ButtonVariant::One(_) => hid.push_input(&play_pause_report),
            ButtonVariant::Two(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Three(_) => hid.push_input(&volume_down_report),
            ButtonVariant::Four(_) => hid.push_input(&volume_up_report),
            ButtonVariant::Five(_) => hid.push_input(&play_pause_report),
            ButtonVariant::Six(_) => hid.push_input(&keyboard_report),
        }
    }

    pub fn release_key(
        &self,
        hid: &HIDClass<'static, hal::usb::UsbBus>,
    ) -> Result<usize, UsbError> {
        let keyboard_report = KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        };

        match self {
            ButtonVariant::One(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Two(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Three(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Four(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Five(_) => hid.push_input(&keyboard_report),
            ButtonVariant::Six(_) => hid.push_input(&keyboard_report),
        }
    }
}

impl InputPin for ButtonVariant {
    // alias for the Error type
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        match self {
            ButtonVariant::One(gpio) => gpio.is_high(),
            ButtonVariant::Two(gpio) => gpio.is_high(),
            ButtonVariant::Three(gpio) => gpio.is_high(),
            ButtonVariant::Four(gpio) => gpio.is_high(),
            ButtonVariant::Five(gpio) => gpio.is_high(),
            ButtonVariant::Six(gpio) => gpio.is_high(),
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        match self {
            ButtonVariant::One(gpio) => gpio.is_low(),
            ButtonVariant::Two(gpio) => gpio.is_low(),
            ButtonVariant::Three(gpio) => gpio.is_low(),
            ButtonVariant::Four(gpio) => gpio.is_low(),
            ButtonVariant::Five(gpio) => gpio.is_low(),
            ButtonVariant::Six(gpio) => gpio.is_low(),
        }
    }
}

// USB HID Usage Tables
// https://www.usb.org/sites/default/files/documents/hut1_12v1.pdf
// Page: 56 for Function Keys.
// E.g Keyboard F13 is 104:(0x68) - decimal:hex
