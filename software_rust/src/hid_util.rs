// TODO: Implement a queue like system for chaining HID reports and for mode switching for different functionality.

use rp_pico::hal;
use usbd_hid::hid_class::HIDClass;

use usbd_hid::descriptor::{KeyboardReport, MediaKey, MediaKeyboardReport};

use heapless::FnvIndexMap;
use heapless::Vec;
// use heapless::spsc::Queue;

use crate::constants::*;

// #[derive(Clone)]
pub struct CustomKeycode {
    array: Vec<u8, BUTTON_COUNT>,
    index_map: FnvIndexMap<u8, u8, BUTTON_COUNT>,
}

impl CustomKeycode {
    pub fn new() -> Self {
        CustomKeycode {
            array: Vec::new(),
            index_map: FnvIndexMap::new(),
        }
    }

    pub fn get_array(&mut self) -> [u8; 6] {
        let mut cloned_vec = self.array.clone();
        cloned_vec.resize(6, 0).unwrap();
        cloned_vec.into_array().unwrap()
    }

    pub fn append(&mut self, keycode: u8) {
        self.array.push(keycode).unwrap();
        self.index_map
            .insert(keycode, self.array.len() as u8)
            .unwrap();
    }
}

pub struct HIDUtil {
    pub custom_keycode: CustomKeycode,
    hid: &'static HIDClass<'static, hal::usb::UsbBus>,
    // custom flag for keyboardreport and mediareport
}

impl HIDUtil {
    pub fn new(hid: &'static HIDClass<'static, hal::usb::UsbBus>) -> Self {
        HIDUtil {
            custom_keycode: CustomKeycode::new(),
            hid,
        }
    }

    pub fn send_keyboard_report(&self, report: KeyboardReport) {
        let _ = self.hid.push_input(&report);
    }
}
