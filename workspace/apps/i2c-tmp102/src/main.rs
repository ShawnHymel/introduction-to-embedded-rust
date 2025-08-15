#![no_std]
#![no_main]

// Alias our HAL
use rp2040_hal as hal;

// We need to write our own panic handler
use core::panic::PanicInfo;

// Bring GPIO structs/functions into scope
use hal::gpio::{Pin, FunctionI2C};

// USB device and Communications Class Device (CDC) support
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

// I2C structs/functions
use embedded_hal::i2c::I2c;

// Used for the rate/frequency type
use hal::fugit::RateExtU32;

// For working with non-heap strings
use heapless::String;
use core::fmt::Write;

// Custom panic handler: just loop forever
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Copy bootloader from rp2040-boot2 into BOOT2 section of memory
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// Constants
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;  // External crystal on board
const TMP102_ADDR: u8 = 0x48;               // Device address on bus
const TMP102_REG_TEMP: u8 = 0x0;            // Address of temperature register

// Main entrypoint (custom defined for embedded targets)
#[hal::entry]
fn main() -> ! {
    // Get ownership of hardware peripherals
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog and clocks
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Single-cycle I/O block (fast GPIO)
    let sio = hal::Sio::new(pac.SIO);

    // Split off ownership of Peripherals struct, set pins to default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure I2C pins
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio19.reconfigure();

    // Initialize and take ownership of the I2C peripheral
    let mut i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // Move ownership of TIMER peripheral to create Timer struct
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Initialize the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Configure the USB as CDC
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID/PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    
    // Read buffer
    let mut rx_buf = [0u8; 2];
    let mut output = String::<64>::new();

    // Superloop
    let mut timestamp = timer.get_counter();
    loop {
        // Needs to be called at least every 10 ms
        let _ = usb_dev.poll(&mut [&mut serial]);

        // Read from I2C every second (non-blocking)
        if (timer.get_counter() - timestamp).to_millis() >= 1_000 {
            timestamp = timer.get_counter();
            
            // Read from sensor
            let result = i2c.write_read(
                TMP102_ADDR, 
                &[TMP102_REG_TEMP],
                &mut rx_buf,
            ); 
            if result.is_err() {
                let _ = serial.write(b"ERROR: Could not read temperature\r\n");
                continue;
            }

            // Convert raw reading (signed 12-bit value) into Celsius
            let temp_raw = ((rx_buf[0] as u16) << 8) | (rx_buf[1] as u16);
            let temp_signed = (temp_raw as i16) >> 4;
            let temp_c = (temp_signed as f32) * 0.0625;
            
            // Print out value
            output.clear();
            write!(&mut output, "Temperature: {:.2} deg C\r\n", temp_c).unwrap();
            let _ = serial.write(output.as_bytes());
        }
    }
}
