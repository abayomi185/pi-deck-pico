#![no_main]
#![no_std]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

mod button;
mod debouncer;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true)]
mod app {

    use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
    // Time handling traits
    use embedded_hal::timer::CountDown;

    // use embedded_time::rate::Extensions;
    use fugit::ExtU32;
    use fugit::MicrosDurationU32;
    use fugit::MillisDurationU32;
    use fugit::RateExtU32;

    use fugit::SecsDurationU32;
    use heapless::String;
    // use nb;

    // A shorter alias for the Peripheral Access Crate, which provides low-level
    // register access
    // use rp_pico::hal::pac;
    // A shorter alias for the Hardware Abstraction Layer, which provides
    // higher-level drivers.
    use rp_pico::hal;
    use rp_pico::hal::timer::Alarm;
    use rp_pico::pac;
    use rp_pico::XOSC_CRYSTAL_FREQ; // Directly imported

    // USB Device support
    use usb_device::{class_prelude::*, prelude::*};
    // USB Communications Class Device support
    use usbd_serial::SerialPort;
    // USB HID Class Device support
    use usbd_hid::descriptor::generator_prelude::*;
    use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport};
    use usbd_hid::hid_class::HIDClass;

    use embedded_graphics::{
        image::{Image, ImageRaw},
        pixelcolor::BinaryColor,
        prelude::*,
    };
    use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

    type DisplayI2C = hal::I2C<
        pac::I2C0,
        (
            hal::gpio::Pin<hal::gpio::bank0::Gpio0, hal::gpio::Function<hal::gpio::I2C>>,
            hal::gpio::Pin<hal::gpio::bank0::Gpio1, hal::gpio::Function<hal::gpio::I2C>>,
        ),
    >;

    use crate::button::Button;
    use crate::button::ButtonVariant;

    // Blink time 5 seconds
    // const SCAN_TIME_US: u32 = 12000000;
    const SCAN_TIME_US: SecsDurationU32 = SecsDurationU32::secs(12);
    const DEBOUNCE_TIME: MicrosDurationU32 = MicrosDurationU32::micros(26_000);

    #[shared]
    struct Shared {
        timer: hal::timer::Timer,
        alarm0: hal::timer::Alarm0,
        alarm1: hal::timer::Alarm1,
        alarm2: hal::timer::Alarm2,
        alarm3: hal::timer::Alarm3,
        // i2c: hal::i2c::I2C<i2c0, Pin>,
        // display: ssd1306::Ssd1306<
        //     ssd1306::I2CDisplayInterface,
        //     ssd1306::size::DisplaySize128x32,
        //     ssd1306::rotation::DisplayRotation,
        // >,
        display: Ssd1306<
            I2CInterface<DisplayI2C>,
            DisplaySize128x32,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x32>,
        >,
        serial: SerialPort<'static, hal::usb::UsbBus>,
        usb_hid: HIDClass<'static, hal::usb::UsbBus>,
        usb_dev: usb_device::device::UsbDevice<'static, hal::usb::UsbBus>,
        button_array: [Button; 6],
        led: hal::gpio::Pin<hal::gpio::pin::bank0::Gpio25, hal::gpio::ReadableOutput>,
        led_blink_enable: bool,
    }

    #[local]
    struct Local {}

    #[init(local = [usb_bus: Option<usb_device::bus::UsbBusAllocator<hal::usb::UsbBus>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        //*******
        // Initialization of the system clock.
        let mut resets = ctx.device.RESETS;
        let mut watchdog = hal::watchdog::Watchdog::new(ctx.device.WATCHDOG);

        // Configure the clocks - The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        //*******
        // Initialization of the USB and Serial and USB Device ID.
        // USB
        //
        // Set up the USB driver
        // The bus that is used to manage the device and class below.
        let usb_bus: &'static _ =
            ctx.local
                .usb_bus
                .insert(UsbBusAllocator::new(hal::usb::UsbBus::new(
                    ctx.device.USBCTRL_REGS,
                    ctx.device.USBCTRL_DPRAM,
                    clocks.usb_clock,
                    true,
                    &mut resets,
                )));

        // Set up the USB Communications Class Device driver.
        let serial = SerialPort::new(usb_bus);
        let usb_hid = HIDClass::new(usb_bus, MediaKeyboardReport::desc(), 60);

        // Create a USB device with a fake VID and PID
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("YomiTosh")
            .product("Pi Deck Pico")
            .serial_number("D001")
            .device_class(2) // from: https://www.usb.org/defined-class-codes
            .build();

        let mut timer = hal::Timer::new(ctx.device.TIMER, &mut resets);
        let mut alarm0 = timer.alarm_0().unwrap();
        let _ = alarm0.schedule(SCAN_TIME_US);
        // let _ = alarm0.schedule();
        alarm0.enable_interrupt();
        let alarm1 = timer.alarm_1().unwrap();
        let alarm2 = timer.alarm_2().unwrap();
        let alarm3 = timer.alarm_3().unwrap();
        // Consider using a shared delay in future
        // let mut delay = timer.count_down();

        let sio = hal::Sio::new(ctx.device.SIO);
        let pins = hal::gpio::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        // let mut led = pins.gpio25.into_push_pull_output();
        let mut led = pins.gpio25.into_readable_output();
        led.set_high().unwrap();
        // led.into_readable_output();

        let led_blink_enable = true;

        let i2c = hal::i2c::I2C::i2c0(
            ctx.device.I2C0,
            pins.gpio0.into_mode(), // SDA
            pins.gpio1.into_mode(), // SCL
            400.kHz(),
            &mut resets,
            &clocks.system_clock,
        );

        // Add instantiated Ssd1306 type to shared
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        // Create raw with imagemagick
        // https://github.com/jamwaffles/ssd1331/issues/10#issuecomment-787125252
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
        let im = Image::new(&raw, Point::new(32, 0));
        im.draw(&mut display).unwrap();
        display.flush().unwrap();

        let button_array: [Button; 6] = [
            Button::new(ButtonVariant::One(pins.gpio26.into_mode())),
            Button::new(ButtonVariant::Two(pins.gpio27.into_mode())),
            Button::new(ButtonVariant::Three(pins.gpio28.into_mode())),
            Button::new(ButtonVariant::Four(pins.gpio4.into_mode())),
            Button::new(ButtonVariant::Five(pins.gpio3.into_mode())),
            Button::new(ButtonVariant::Six(pins.gpio2.into_mode())),
        ];

        for button in button_array.iter() {
            button.variant.set_button_low_interrupt(true)
        }

        (
            Shared {
                timer,
                alarm0,
                alarm1,
                alarm2,
                alarm3,
                display,
                serial,
                usb_hid,
                usb_dev,
                button_array,
                led,
                led_blink_enable,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[idle(shared = [button_array])]
    fn idle(_ctx: idle::Context) -> ! {
        // (_ctx.shared.button_array).lock(|button_array_a| {
        //     for button in button_array_a.iter_mut() {
        //         button.variant.set_button_low_interrupt(true)
        //     }
        // });

        loop {
            cortex_m::asm::nop();
        }
    }

    // #[task(local = [])]
    // fn test_local_task(ctx: test_local_task::Context) {}

    // #[task(shared = [])]
    // fn test_shared_task(mut ctx: test_shared_task::Context) {}

    #[task(binds = USBCTRL_IRQ, priority = 3, shared = [serial, usb_dev, usb_hid])]
    fn usb_rx(ctx: usb_rx::Context) {
        let usb_dev = ctx.shared.usb_dev;
        let serial = ctx.shared.serial;
        let usb_hid = ctx.shared.usb_hid;

        (serial, usb_dev, usb_hid).lock(|serial_a, usb_dev_a, usb_hid_a| {
            if usb_dev_a.poll(&mut [serial_a, usb_hid_a]) {
                let mut buf = [0u8; 64];
                match serial_a.read(&mut buf) {
                    Err(_e) => {
                        //Do Nothing
                        // let _ = serial_a.write(b"Error.")
                        // let _ = serial_a.flush()
                    }
                    Ok(0) => {
                        // Do nothing
                        let _ = serial_a.write(b"Didn't received data.");
                        let _ = serial_a.flush();
                    }
                    Ok(_count) => {
                        // match_usb_serial_buf(&buf, led_a, led_blink_enable_a, serial_a, counter_a);
                        write_serial(serial_a, "I'm here", false)
                    }
                }
                // TODO: USB HID action here
                // usb_hid_a.match
                // let key = MediaKey
            }
        })
    }

    #[task(
        binds = IO_IRQ_BANK0,
        priority = 4,
        shared = [led, serial, timer, alarm1, alarm2, button_array, usb_hid]
    )]
    fn handle_button(ctx: handle_button::Context) {
        let led = ctx.shared.led;
        let button_array = ctx.shared.button_array;

        let usb_hid = ctx.shared.usb_hid;
        let serial = ctx.shared.serial;

        let timer = ctx.shared.timer;
        let alarm1 = ctx.shared.alarm1;
        let alarm2 = ctx.shared.alarm2;

        (serial, timer, alarm1, alarm2, button_array, led, usb_hid).lock(
            |serial_a, timer_a, alarm_a, alarm_b, button_array_a, led_a, usb_hid_a| {
                // TODO: This is running multiple times, not expected behaviour.
                // It does not break and turns off led as it turns it on except last key.
                // Return boolean from is_low and use it to determine break.
                // To possibly detect multiple keys, save keys pressed into array then act
                // on it later.

                for (index, button) in button_array_a.iter_mut().enumerate() {
                    let mut serial_message: String<8> = String::new();
                    let mut serial_message_2: String<8> = String::new();
                    let mut serial_message_3: String<8> = String::new();
                    let mut serial_message_4: String<8> = String::new();

                    let index_string: String<1> = String::from(index as u8);

                    serial_message.push_str("\nKey ").unwrap();
                    serial_message.push_str(&index_string).unwrap();
                    // write_serial(serial_a, serial_message.as_str(), false);

                    // TODO: the raw value is always 0 when the interrupt is triggered
                    // This does not allow the debouncer to reset its state
                    let button_state = button.variant.is_low().unwrap();
                    button.debounce(timer_a, button_state);

                    let mut count_down = timer_a.count_down();
                    count_down.start(DEBOUNCE_TIME);
                    let _ = nb::block!(count_down.wait());

                    button.debounce(timer_a, button_state);
                    // if button_state {
                    //     serial_message_4
                    //         .push_str(&String::<32>::from("LOW_"))
                    //         .unwrap();
                    //     write_serial(serial_a, serial_message_4.as_str(), false);
                    // } else {
                    //     serial_message_4
                    //         .push_str(&String::<32>::from("HIGH_"))
                    //         .unwrap();
                    //     write_serial(serial_a, serial_message_4.as_str(), false);
                    // }

                    if !button.is_pressed && button.to_be_released {
                        button.to_be_released = false;

                        serial_message_3
                            .push_str(&String::<32>::from("B_"))
                            .unwrap();
                        write_serial(serial_a, serial_message_3.as_str(), false);

                        //     let _ = led_a.toggle();
                        // button.variant.clear_button_high_interrupt();
                        button.variant.set_button_high_interrupt(false);
                        // button.variant.set_button_low_interrupt(true);
                    }

                    if button.is_pressed {
                        // improvements using this but there are numerous miss clicks and delayed presses
                        button.reset();

                        // let mut count_down = timer_a.count_down();
                        // count_down.start(200.millis());
                        // let _ = nb::block!(count_down.wait());

                        serial_message_2
                            .push_str(&String::<32>::from("A_"))
                            .unwrap();
                        write_serial(serial_a, serial_message_2.as_str(), false);

                        let _ = led_a.toggle();
                        // button.variant.clear_button_low_interrupt();
                        // button.variant.set_button_low_interrupt(false);
                        button.variant.set_button_high_interrupt(true);

                        // let mut count_down = timer_a.count_down();
                        // count_down.start(100.millis());
                        // let _ = nb::block!(count_down.wait());
                        button.to_be_released = true;
                    }
                }

                // for button in button_array_a.iter_mut() {
                //     button.variant.set_button_low_interrupt(true);
                // }

                for button in button_array_a.iter_mut() {
                    button.variant.clear_button_low_interrupt();
                }
            },
        );
    }

    //This works - timer_irq; LED light turns off after SCAN_TIME_US
    #[task(
        binds = TIMER_IRQ_0,
        priority = 1,
        shared = [timer, alarm0, led, led_blink_enable, serial],
        local = [tog: bool = true]
    )]
    fn timer_irq(mut ctx: timer_irq::Context) {
        let buf = [0u8; 64];

        let led = ctx.shared.led;
        let led_blink_enable = ctx.shared.led_blink_enable;
        let tog = ctx.local.tog;

        (led, led_blink_enable).lock(|led_a, led_blink_enable_a| {
            if *led_blink_enable_a {
                if *tog {
                    led_a.set_low().unwrap();
                } else {
                    led_a.set_high().unwrap();
                }
            }
        });

        // Clears the timer interrupt and Set's the new delta_time in the future.
        // let mut timer = ctx.shared.timer;
        let mut alarm = ctx.shared.alarm0;
        (alarm).lock(|alarm_a| {
            // (timer).lock(|timer_a| {
            alarm_a.clear_interrupt();
            let _ = alarm_a.schedule(SCAN_TIME_US);
            // });
        });

        // Write the message "blabla! 2" do USB-Serial.
        ctx.shared.serial.lock(|s| {
            write_serial(s, unsafe { core::str::from_utf8_unchecked(&buf) }, false);
        });
    }

    /// This function come from the github with USB-Serial example (see link above).
    ///
    /// Helper function to ensure all data is written across the serial interface.
    fn write_serial(serial: &mut SerialPort<'static, hal::usb::UsbBus>, buf: &str, block: bool) {
        let write_ptr = buf.as_bytes();

        // Because the buffer is of constant size and initialized to zero (0) we here
        // add a test to determine the size that's really occupied by the str that we
        // wan't to send. From index zero to first byte that is as the zero byte value.
        let mut index = 0;
        while index < write_ptr.len() && write_ptr[index] != 0 {
            index += 1;
        }
        let mut write_ptr = &write_ptr[0..index];

        while !write_ptr.is_empty() {
            match serial.write(write_ptr) {
                Ok(len) => write_ptr = &write_ptr[len..],
                // Meaning the USB write buffer is full
                Err(UsbError::WouldBlock) => {
                    if !block {
                        break;
                    }
                }
                // On error, just drop unwritten data.
                Err(_) => break,
            }
        }
        // let _ = serial.write("\n".as_bytes());
        let _ = serial.flush();
    }
}
