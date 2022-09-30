// Testing special keywords
// TODO: yo : this is a todo
// FIXME: hi : I'm likely to break
// BUG: hi : I'm broken
// NOTE: hi : I'm a note
// HACK: hi : I'm a hack
// XXX: hi : I'm a warning
// WARNING: hi : I'm a warning
// OPTIMIZE: hi : I'm an optimization
// WARN: hi : I'm a warning
// PERF: hi : I'm a performance issue
// FIX: hi : I need to be fixed

pub const BUTTON_COUNT: usize = 6;
pub const INDEX_MAP_SIZE: usize = 8; // Must be power of 2

pub const KEYCODE_1: u8 = 0x69;
pub const KEYCODE_2: u8 = 0x6A;
pub const KEYCODE_3: u8 = 0x6B;
pub const KEYCODE_4: u8 = 0x6C;
pub const KEYCODE_5: u8 = 0x6D;
pub const KEYCODE_6: u8 = 0x6E;

pub const MEDIAKEY_PLAYPAUSE: u8 = 0xCD;
pub const MEDIAKEY_VOLUP: u8 = 0xE9;
pub const MEDIAKEY_VOLDOWN: u8 = 0xEA;
#[allow(dead_code)]
pub const MEDIAKEY_MUTE: u8 = 0xB2;
pub const MEDIAKEY_PREVTRACK: u8 = 0xB6;
pub const MEDIAKEY_NEXTTRACK: u8 = 0xB5;
pub const MEDIAKEY_NONE: u8 = 0x00;

pub const KEY_MODE_BUTTONS: [u8; 2] = [KEYCODE_1, KEYCODE_2];
pub const MEDIA_MODE_BUTTONS: [u8; 2] = [MEDIAKEY_PLAYPAUSE, MEDIAKEY_NONE];
