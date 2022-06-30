#![no_main]
#![no_std]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true)]
mod app {
    use embedded_hal::digital::v2::OutputPin;
    // Time handling traits
    use embedded_time::duration::Extensions;

    use rp_pico::hal::prelude::*;
    // A shorter alias for the Peripheral Access Crate, which provides low-level
    // register access
    // use rp_pico::hal::pac;
    // A shorter alias for the Hardware Abstraction Layer, which provides
    // higher-level drivers.
    use rp_pico::hal;
    use rp_pico::pac;
    use rp_pico::XOSC_CRYSTAL_FREQ; // Directly imported

    // USB Device support
    use usb_device::{class_prelude::*, prelude::*};
    // USB Communications Class Device support
    use usbd_serial::SerialPort;

    // Blink time 5 seconds
    const SCAN_TIME_US: u32 = 5000000;

    #[shared]
    struct Shared {
        timer: hal::Timer,
        alarm: hal::timer::Alarm0,
        // led: hal::gpio::Pin<hal::gpio::pin::bank0::Gpio25, hal::gpio::PushPullOutput>,
        // led_blink_enable: bool,
        serial: SerialPort<'static, hal::usb::UsbBus>,
        usb_dev: usb_device::device::UsbDevice<'static, hal::usb::UsbBus>,
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

        (
            Shared {
                timer,
                alarm,
                serial,
                usb_dev,
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
}
