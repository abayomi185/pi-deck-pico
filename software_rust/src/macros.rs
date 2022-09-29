#[macro_export]
macro_rules! gen_keyboard_report {
    ($usage_id:expr) => {
        match $usage_id {
            0 => KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            },
            _ => KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [$usage_id, 0, 0, 0, 0, 0],
            },
        }
    };

    (@array $array_val:expr) => {
        KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: $array_val,
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
