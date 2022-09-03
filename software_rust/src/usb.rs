fn send_mouse_report(
    mut shared_hid: impl Mutex<T = HIDClass<'static, usb::UsbBusType>>,
    x: i8,
    y: i8,
    buttons: u8,
) {
    let mr = MouseReport {
        x,
        y,
        buttons,
        wheel: 0,
        pan: 0,
    };

    shared_hid.lock(|hid| {
        rprintln!("Sending mouse report...");
        hid.push_input(&mr).ok();
    });
}
