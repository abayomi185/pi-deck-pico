use embedded_hal::digital::v2::InputPin;

use rp_pico::hal;
use rp_pico::hal::gpio::Interrupt::{EdgeHigh, EdgeLow};

use usbd_hid::descriptor::{KeyboardReport, MediaKey, MediaKeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;
use usbd_hid::UsbError;

use heapless::Vec;

use crate::debouncer::Debouncer;
use crate::key_config::KeyConfig;
use crate::{gen_keyboard_report, gen_media_report};

pub struct Button {
    pub variant: ButtonVariant,
    debouncer: Debouncer,
    pub is_pressed: bool,
    pub to_be_released: bool,
}

impl Button {
    pub fn new(variant: ButtonVariant) -> Self {
        Self {
            variant,
            // 25ms debounce
            debouncer: Debouncer::new(25_000),
            is_pressed: false,
            to_be_released: false,
        }
    }

    pub fn debounce(&mut self, timer: &hal::timer::Timer, current_state: bool) {
        let current_time = timer.get_counter_low();
        self.debouncer.update(current_time, current_state);
        self.is_pressed = self.debouncer.stabilised_state;
        // self.is_released = {
        //     let mut val = false;
        //     if !self.debouncer.stabilised_state && self.debouncer.is_debounced {
        //         val = true
        //     }
        //     val
        // }
    }

    pub fn reset(&mut self) {
        self.debouncer.current_state = false;
        self.debouncer.stabilised_state = false;
        self.is_pressed = false;
    }

    pub fn is_released(&mut self) -> bool {
        if !self.debouncer.stabilised_state && !self.debouncer.current_state {
            return true;
        }
        false
    }

    // pub fn is_pressed(&mut self) {
    //     if self.debouncer.stabilised_state {
    //         self.is_pressed = true;
    //         self.is_released = false;
    //     } else {
    //         self.is_released = true;
    //         self.is_pressed = false;
    //     }
    // }
}

// TODO: Implement a stack for concurrently pressed buttons to send via usb_hid
struct ReportStack {
    stack: Vec<KeyboardReport, 6>,
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
    One {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio26, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
    Two {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio27, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
    Three {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio28, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
    Four {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio4, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
    Five {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
    Six {
        gpio: hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::FloatingInput>,
        id: KeyConfig,
    },
}

impl ButtonVariant {
    // TODO: I can use traits so I don't have to create and redeclare the gpio attr.
    // Would've saved a lot of time.

    pub fn set_button_low_interrupt(&self, set_state: bool) {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
            ButtonVariant::Two { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
            ButtonVariant::Three { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
            ButtonVariant::Four { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
            ButtonVariant::Five { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
            ButtonVariant::Six { gpio, .. } => gpio.set_interrupt_enabled(EdgeLow, set_state),
        }
    }

    pub fn set_button_high_interrupt(&self, set_state: bool) {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
            ButtonVariant::Two { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
            ButtonVariant::Three { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
            ButtonVariant::Four { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
            ButtonVariant::Five { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
            ButtonVariant::Six { gpio, .. } => gpio.set_interrupt_enabled(EdgeHigh, set_state),
        }
    }

    pub fn clear_button_low_interrupt(&mut self) {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Two { gpio, .. } => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Three { gpio, .. } => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Four { gpio, .. } => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Five { gpio, .. } => gpio.clear_interrupt(EdgeLow),
            ButtonVariant::Six { gpio, .. } => gpio.clear_interrupt(EdgeLow),
        }
    }

    pub fn clear_button_high_interrupt(&mut self) {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
            ButtonVariant::Two { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
            ButtonVariant::Three { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
            ButtonVariant::Four { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
            ButtonVariant::Five { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
            ButtonVariant::Six { gpio, .. } => gpio.clear_interrupt(EdgeHigh),
        }
    }

    pub fn send_key(&self, hid: &HIDClass<'static, hal::usb::UsbBus>) -> Result<usize, UsbError> {
        match self {
            ButtonVariant::One { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
            ButtonVariant::Two { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
            ButtonVariant::Three { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
            ButtonVariant::Four { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
            ButtonVariant::Five { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
            ButtonVariant::Six { id, .. } => hid.push_input(&gen_keyboard_report!(0x69)),
        }
    }

    pub fn release_key(
        &self,
        hid: &HIDClass<'static, hal::usb::UsbBus>,
    ) -> Result<usize, UsbError> {
        match self {
            ButtonVariant::One { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
            ButtonVariant::Two { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
            ButtonVariant::Three { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
            ButtonVariant::Four { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
            ButtonVariant::Five { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
            ButtonVariant::Six { .. } => hid.push_input(&gen_keyboard_report!(0x0)),
        }
    }
}

impl InputPin for ButtonVariant {
    // alias for the Error type
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.is_high(),
            ButtonVariant::Two { gpio, .. } => gpio.is_high(),
            ButtonVariant::Three { gpio, .. } => gpio.is_high(),
            ButtonVariant::Four { gpio, .. } => gpio.is_high(),
            ButtonVariant::Five { gpio, .. } => gpio.is_high(),
            ButtonVariant::Six { gpio, .. } => gpio.is_high(),
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        match self {
            ButtonVariant::One { gpio, .. } => gpio.is_low(),
            ButtonVariant::Two { gpio, .. } => gpio.is_low(),
            ButtonVariant::Three { gpio, .. } => gpio.is_low(),
            ButtonVariant::Four { gpio, .. } => gpio.is_low(),
            ButtonVariant::Five { gpio, .. } => gpio.is_low(),
            ButtonVariant::Six { gpio, .. } => gpio.is_low(),
        }
    }
}

// USB HID Usage Tables
// https://www.usb.org/sites/default/files/documents/hut1_12v1.pdf
// Page: 56 for Function Keys.
// E.g Keyboard F13 is 104:(0x68) - decimal:hex
