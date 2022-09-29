// TODO: Implement a queue like system for chaining HID reports and for mode switching for different functionality.

use enum_map::EnumMap;
use rp_pico::hal;
use usbd_hid::hid_class::HIDClass;
use usbd_hid::UsbError;

use crate::{gen_keyboard_report, gen_media_report};
use usbd_hid::descriptor::{KeyboardReport, MediaKey, MediaKeyboardReport};

use heapless::FnvIndexMap;
use heapless::Vec;
// use heapless::spsc::Queue;

use crate::constants::*;
use crate::key_config::{KeyConfig, KeyMode};

// #[derive(Clone)]
pub struct CustomKeycode {
    // array: Vec<u8, BUTTON_COUNT>, // Might still need this for keycode array if index_map does not suffice
    index_map: FnvIndexMap<u8, bool, BUTTON_COUNT>,
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

pub struct HIDUtil {
    pub custom_keycode: CustomKeycode,
    key_config: EnumMap<KeyConfig, [u8; 2]>,
    hid_keyboard: &'static HIDClass<'static, hal::usb::UsbBus>,
    hid_media: &'static HIDClass<'static, hal::usb::UsbBus>,
    mode: KeyMode, // mode flag for keyboardreport and mediareport
}

impl HIDUtil {
    pub fn new(
        hid_keyboard: &'static HIDClass<'static, hal::usb::UsbBus>,
        hid_media: &'static HIDClass<'static, hal::usb::UsbBus>,
    ) -> Self {
        HIDUtil {
            custom_keycode: CustomKeycode::new(),
            key_config: KeyConfig::new(),
            hid_keyboard,
            hid_media,
            mode: KeyMode::Keyboard,
        }
    }

    pub fn push_input(&mut self, button_id: KeyConfig) {
        match self.mode {
            KeyMode::Keyboard => {
                let keycode = self.key_config[button_id][0];
                self.custom_keycode.index_map.insert(keycode, true).unwrap();
                // If mode changes, do nothing
                if !self.has_mode_changed() {
                    let _ = self
                        .hid_keyboard
                        .push_input(&gen_keyboard_report!(@array self
                        .custom_keycode
                        .get_keycode_array()));
                }
            }
            KeyMode::Media => {
                let media_key = self.key_config[button_id][1];
                let report = gen_media_report!(media_key);
                self.hid_media.push_input(&report).unwrap();
            }
        }
    }

    pub fn release_input(&mut self, button_id: KeyConfig) {
        match self.mode {
            KeyMode::Keyboard => {
                let keycode = self.key_config[button_id][0];
                self.custom_keycode.index_map.remove(&keycode).unwrap();
            }
            KeyMode::Media => {
                let report = gen_media_report!(MEDIAKEY_NONE);
                self.hid_media.push_input(&report).unwrap();
            }
        }
    }

    fn has_mode_changed(&mut self) -> bool {
        let key_status: bool = self
            .custom_keycode
            .index_map
            .iter()
            .map(|(k, _)| MODE_SWITCH_BUTTONS.contains(k))
            .reduce(|a, b| a && b)
            .unwrap();

        if key_status {
            match self.mode {
                KeyMode::Keyboard => self.mode = KeyMode::Media,
                KeyMode::Media => self.mode = KeyMode::Keyboard,
            }
        };
        key_status
    }
}
