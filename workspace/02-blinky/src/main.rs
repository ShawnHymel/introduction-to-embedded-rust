#![no_std]
#![no_main]

use panic_halt as _; // Halts on panic
use rp2040_hal as hal;

use hal::{entry, pac, Sio, Watchdog};
use embedded_hal::digital::OutputPin; // Trait for set_high/set_low
use embedded_hal::delay::DelayNs;   // <-- this enables .delay_ms() on Timer

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000; // Pico's crystal frequency

#[entry]
fn main() -> ! {
    // Grab our peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    // let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog and clocks
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
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

    // Split GPIOs
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // GPIO 25 is the onboard LED
    let mut led_pin = pins.gpio25.into_push_pull_output();

    // 
    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Blink loop
    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}
