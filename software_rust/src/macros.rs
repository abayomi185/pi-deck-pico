#[macro_export]
macro_rules! gen_keyboard_report {
    ($usage_id:expr) => {
        KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [$usage_id, 0, 0, 0, 0, 0],
        }
    };
}

#[macro_export]
macro_rules! gen_media_report {
    ($usage_id:expr) => {
        MediaKeyboardReport {
            usage_id: $usage_id as u16,
        }
    };
}
