// TODO: Change all this to an enum map

use enum_map::{enum_map, Enum, EnumMap};

// use usbd_hid::descriptor::MediaKey;

use crate::constants::*;

pub enum KeyMode {
    Keyboard,
    Media,
}

#[derive(Enum, Clone, Copy)]
pub enum KeyConfig {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl KeyConfig {
    pub fn new() -> EnumMap<KeyConfig, [u8; 2]> {
        enum_map! {
            KeyConfig::One => [KEYCODE_1, MEDIAKEY_PLAYPAUSE],
            KeyConfig::Two => [KEYCODE_2, MEDIAKEY_NONE],
            KeyConfig::Three => [KEYCODE_3, MEDIAKEY_VOLUP],
            KeyConfig::Four => [KEYCODE_4, MEDIAKEY_VOLDOWN],
            KeyConfig::Five => [KEYCODE_5, MEDIAKEY_PREVTRACK],
            KeyConfig::Six => [KEYCODE_6, MEDIAKEY_NEXTTRACK],
        }
    }
}
