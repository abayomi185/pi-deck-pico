#![no_main]
#![no_std]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true)]
mod app {

    use embedded_hal::digital::v2::{InputPin, OutputPin};
    // Time handling traits
    use embedded_time::duration::Extensions as _;
    use embedded_time::rate::Extensions;

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

    // Blink time 5 seconds
    const SCAN_TIME_US: u32 = 2000000;

    #[shared]
    struct Shared {
        timer: hal::Timer,
        alarm: hal::timer::Alarm0,
        // led_blink_enable: bool,
        // pins: hal::gpio::Pins,
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
        usb_dev: usb_device::device::UsbDevice<'static, hal::usb::UsbBus>,
        // input_pin_array: [hal::gpio::Pin<hal::gpio::pin::bank0::Gpio26, hal::gpio::PullUpInput>; 6],
        // input_pin_array: [hal::gpio::pin::bank0::Pins; 6],
        input_pin_array: [hal::gpio::dynpin::DynPin; 6],
        led: hal::gpio::Pin<hal::gpio::pin::bank0::Gpio25, hal::gpio::PushPullOutput>,
        key_one: hal::gpio::Pin<hal::gpio::pin::bank0::Gpio26, hal::gpio::PullUpInput>,
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

        // Create a USB device with a fake VID and PID
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("YomiTosh")
            .product("Pi Deck Pico")
            .serial_number("D001")
            .device_class(2) // from: https://www.usb.org/defined-class-codes
            .build();

        let mut timer = hal::Timer::new(ctx.device.TIMER, &mut resets);
        let mut alarm = timer.alarm_0().unwrap();
        let _ = alarm.schedule(SCAN_TIME_US.microseconds());
        alarm.enable_interrupt();

        let sio = hal::Sio::new(ctx.device.SIO);
        let pins = hal::gpio::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let mut led = pins.gpio25.into_push_pull_output();
        led.set_high().unwrap();

        let i2c = hal::i2c::I2C::i2c0(
            ctx.device.I2C0,
            pins.gpio0.into_mode(), // SDA
            pins.gpio1.into_mode(), // SCL
            400.kHz(),
            &mut resets,
            125_000_000.Hz(),
        );

        // Add instantiated Ssd1306 type to shared
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
        let im = Image::new(&raw, Point::new(32, 0));
        im.draw(&mut display).unwrap();
        display.flush().unwrap();

        // let input_pin_array = [
        //     pins.gpio26.into_pull_up_input(),
        //     pins.gpio27.into_pull_up_input(),
        //     pins.gpio28.into_pull_up_input(),
        //     pins.gpio4.into_pull_up_input(),
        //     pins.gpio3.into_pull_up_input(),
        //     pins.gpio2.into_pull_up_input(),
        // ];

        let mut input_pin_array: [hal::gpio::dynpin::DynPin; 6] = [
            pins.gpio13.into(),
            pins.gpio27.into(),
            pins.gpio28.into(),
            pins.gpio4.into(),
            pins.gpio3.into(),
            pins.gpio2.into(),
        ];

        for pin in input_pin_array.iter_mut() {
            pin.into_pull_up_input();
        }

        let key_one = pins.gpio26.into_pull_up_input();

        (
            Shared {
                timer,
                alarm,
                display,
                serial,
                usb_dev,
                input_pin_array,
                led,
                key_one,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[idle(local = [])]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    // #[task(local = [])]
    // fn test_local_task(ctx: test_local_task::Context) {}

    // #[task(shared = [])]
    // fn test_shared_task(mut ctx: test_shared_task::Context) {}

    #[task(binds = USBCTRL_IRQ, priority = 3, shared = [serial, usb_dev])]
    fn usb_rx(ctx: usb_rx::Context) {
        let usb_dev = ctx.shared.usb_dev;
        let serial = ctx.shared.serial;

        (serial, usb_dev).lock(|serial_a, usb_dev_a| {
            if usb_dev_a.poll(&mut [serial_a]) {
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
                    }
                }
            }
        })
    }

    #[task(binds = IO_IRQ_BANK0, priority = 4, shared = [input_pin_array, led, key_one])]
    fn handle_switch(ctx: handle_switch::Context) {
        let input_pin_array = ctx.shared.input_pin_array;
        let led = ctx.shared.led;
        let key_one = ctx.shared.key_one;

        // for pin in input_pin_array.iter_mut() {
        //     pin.into_pull_up_input();
        // }

        // (input_pin_array, led).lock(|input_pin_array_a, led_a| {
        //     for pin in input_pin_array_a.iter_mut() {
        //         if pin.is_low().unwrap() {
        //             led_a.set_high().unwrap();
        //         } else {
        //             led_a.set_low().unwrap();
        //         }
        //     }
        // });

        (led, key_one).lock(|led_a, key_one_a| {
            if key_one_a.is_low().unwrap() {
                led_a.set_low().unwrap();
            } else {
                led_a.set_high().unwrap();
            }
        });
    }

    #[task(binds = TIMER_IRQ_0, priority = 1, shared = [timer, alarm, led])]
    fn timer_irq(mut ctx: timer_irq::Context) {
        let mut buf = [0u8; 64];

        let led = ctx.shared.led;

        let mut timer = ctx.shared.timer;
        let mut alarm = ctx.shared.alarm;
        (alarm).lock(|a| {
            (timer).lock(|timer_a| {
                a.clear_interrupt();
                let _ = a.schedule(SCAN_TIME_US.microseconds());
            });
        });
    }

    // fn to_dynpin_array() -> [hal::gpio::dynpin::DynPin; 6] {
    //     [
    //         hal::gpio::dypin::DynPin::new(hal::gpio::pin0::P0_26),
    //         hal::gpio::dynpin::DynPin::new(hal::gpio::pin0::P0_27),
    //         hal::gpio::dynpin::DynPin::new(hal::gpio::pin0::P0_28),
    //         hal::gpio::dynpin::DynPin::new(hal::gpio::pin0::P0_4),
    //         hal::gpio::dynpin::DynPin::new(hal::gpio::pin0::P0_3),
    //         hal::gpio::dynpin::DynPin::new(hal::gpio::pin0::P0_2),
    //     ]
    // }
}
