#![no_std]
#![no_main]

// Alias our HAL
use rp235x_hal as hal;

// Import traits for embedded abstractions
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// Debugging output
use defmt::*;
use defmt_rtt as _;

// Let panic_probe handle our panic routine
use panic_probe as _;

// Copy boot metadata to .start_block so Boot ROM knows how to boot our program
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// Set external crystal frequency
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

// Main entrypoint (custom defined for embedded targets)
#[hal::entry]
fn main() -> ! {
    info!("Starting blinky");

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

    // Configure pin, get ownership of that pin
    let mut led_pin = pins.gpio15.into_push_pull_output();

    // Move ownership of TIMER0 peripheral to create Timer struct
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Blink loop
    loop {
        led_pin.set_high().unwrap();
        debug!("LED on");
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        debug!("LED off");
        timer.delay_ms(500);
    }
}
