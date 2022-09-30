// TODO: Implement a queue like system for chaining HID reports and for mode switching for different functionality.

use enum_map::EnumMap;
use rp_pico::hal;
use usbd_hid::hid_class::HIDClass;
// use usbd_hid::UsbError;

use crate::{gen_keyboard_report, gen_media_report};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
type DisplayI2C = hal::I2C<
    hal::pac::I2C0,
    (
        hal::gpio::Pin<hal::gpio::bank0::Gpio0, hal::gpio::Function<hal::gpio::I2C>>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio1, hal::gpio::Function<hal::gpio::I2C>>,
    ),
>;

use heapless::FnvIndexMap;
use heapless::String;
use heapless::Vec;
// use heapless::spsc::Queue;

use crate::constants::*;
use crate::display;
use crate::key_config::{KeyConfig, KeyMode};

// #[derive(Clone)]
pub struct CustomKeycode {
    // array: Vec<u8, BUTTON_COUNT>, // Might still need this for keycode array if index_map does not suffice
    index_map: FnvIndexMap<u8, bool, INDEX_MAP_SIZE>,
}

// Just use a hashmap only and iterate through to conver to array

impl CustomKeycode {
    pub fn new() -> Self {
        CustomKeycode {
            // array: Vec::new(),
            index_map: FnvIndexMap::new(),
        }
    }

    fn get_keycode_array(&mut self) -> [u8; 6] {
        let mut array_vec = self
            .index_map
            .iter()
            .map(|(k, _)| *k)
            .collect::<Vec<u8, 6>>();
        let _ = array_vec.resize(BUTTON_COUNT, 0);
        array_vec.into_array().unwrap()
    }
}

impl Default for CustomKeycode {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HIDUtil {
    pub custom_keycode: CustomKeycode,
    key_config: EnumMap<KeyConfig, [u8; 2]>,
    // hid_keyboard: &'static HIDClass<'static, hal::usb::UsbBus>,
    // hid_media: &'static HIDClass<'static, hal::usb::UsbBus>,
    mode: KeyMode, // mode flag for keyboardreport and mediareport
}

impl HIDUtil {
    pub fn new(// hid_keyboard: &'static HIDClass<'static, hal::usb::UsbBus>,
        // hid_media: &'static HIDClass<'static, hal::usb::UsbBus>,
    ) -> Self {
        HIDUtil {
            custom_keycode: CustomKeycode::new(),
            key_config: KeyConfig::new(),
            // hid_keyboard,
            // hid_media,
            mode: KeyMode::Keyboard,
        }
    }

    pub fn push_input(
        &mut self,
        hid_keyboard: &HIDClass<'static, hal::usb::UsbBus>,
        hid_media: &HIDClass<'static, hal::usb::UsbBus>,
        button_id: KeyConfig,
        display: &mut Ssd1306<
            I2CInterface<DisplayI2C>,
            DisplaySize128x32,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x32>,
        >,
    ) {
        match self.mode {
            KeyMode::Keyboard => {
                let keycode = self.key_config[button_id][0];
                self.custom_keycode.index_map.insert(keycode, true).unwrap();

                // If mode changes, release all keys
                if self.is_mode_switch_pressed() {
                    let _ = hid_keyboard.push_input(&gen_keyboard_report!(@array [0; 6]));
                    self.custom_keycode.index_map.clear();
                    self.change_mode();
                    return;
                }

                let _ = hid_keyboard.push_input(&gen_keyboard_report!(@array self
                        .custom_keycode
                        .get_keycode_array()));
            }
            KeyMode::Media => {
                let media_key = self.key_config[button_id][1];
                self.custom_keycode
                    .index_map
                    .insert(media_key, true)
                    .unwrap();

                // If mode changes, release all keys
                if self.is_mode_switch_pressed() {
                    let _ = hid_media.push_input(&gen_media_report!(MEDIAKEY_NONE as u16));
                    self.custom_keycode.index_map.clear();
                    self.change_mode();
                    return;
                }

                let _ = hid_keyboard.push_input(&gen_media_report!(
                    *self.custom_keycode.index_map.last().unwrap().0 as u16 // media_key as u16
                ));
            }
        }

        // Testing with display
        display::show_text(display, "pushed")
    }

    pub fn release_input(
        &mut self,
        hid_keyboard: &HIDClass<'static, hal::usb::UsbBus>,
        hid_media: &HIDClass<'static, hal::usb::UsbBus>,
        button_id: KeyConfig,
        display: &mut Ssd1306<
            I2CInterface<DisplayI2C>,
            DisplaySize128x32,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x32>,
        >,
    ) {
        match self.mode {
            KeyMode::Keyboard => {
                let keycode = self.key_config[button_id][0];

                // Error check - print keycode to display
                // let mut error_message: String<8> = String::new();
                // let keycode_string: String<1> = String::from(keycode as u8);
                // error_message.push_str(&keycode_string).unwrap();
                // display::show_text(display, error_message.as_str());

                if !self.custom_keycode.index_map.is_empty() {
                    self.custom_keycode.index_map.remove(&keycode);
                    let _ = hid_keyboard.push_input(
                        &gen_keyboard_report!(@array self.custom_keycode.get_keycode_array()),
                    );
                }
            }
            KeyMode::Media => {
                let keycode = self.key_config[button_id][1];

                if !self.custom_keycode.index_map.is_empty() {
                    self.custom_keycode.index_map.remove(&keycode);
                    let _ = hid_media.push_input(&gen_media_report!(MEDIAKEY_NONE as u16));
                }
            }
        }

        // Testing with display
        display::show_text(display, "released")
    }

    fn is_mode_switch_pressed(&mut self) -> bool {
        if self.custom_keycode.index_map.len() > 1 {
            let key_status: bool = self
                .custom_keycode
                .index_map
                .iter()
                .map(|(key, _)| KEY_MODE_BUTTONS.contains(key) || MEDIA_MODE_BUTTONS.contains(key))
                // .map(|(key, _)| KEY_MODE_BUTTONS.contains(key))
                .reduce(|a, b| a && b)
                // .unwrap_or(false);
                .unwrap();

            // if key_status {
            //     // WARN: Release all keys when mode changes
            //     // self.custom_keycode.index_map.clear();

            //     match self.mode {
            //         KeyMode::Keyboard => self.mode = KeyMode::Media,
            //         KeyMode::Media => self.mode = KeyMode::Keyboard,
            //     }
            // };

            return key_status;
            // return true;
        }
        // If condition not met, return false
        false
    }

    fn change_mode(&mut self) {
        match self.mode {
            KeyMode::Keyboard => self.mode = KeyMode::Media,
            KeyMode::Media => self.mode = KeyMode::Keyboard,
        }
    }
}

impl Default for HIDUtil {
    fn default() -> Self {
        Self::new()
    }
}
