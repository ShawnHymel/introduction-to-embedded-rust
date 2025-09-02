#![no_std]
#![no_main]

// We need to write our own panic handler
use core::panic::PanicInfo;

// Let us modify data with only immutable reference (enforce borrow rules at runtime)
use core::cell::RefCell;

// Embedded mutex (no threads): access to data by one piece of code at a time
use critical_section::Mutex;

// Alias our HAL
use rp235x_hal as hal;

// Import traits for embedded abstractions
use embedded_hal::digital::StatefulOutputPin;

// Imports for the pin definition
use hal::gpio::{FunctionSio, Pin, PullDown, SioOutput};

// Imports for the timer interrupt
use hal::pac::interrupt;
use hal::timer::{Alarm, Alarm0, CopyableTimer0};

// Direct access to the nested vectored interrupt controller (NVIC)
use cortex_m::peripheral::NVIC;

// Help with timing and duration
use fugit::MicrosDurationU32;

// Custom panic handler: just loop forever
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Copy boot metadata to .start_block so Boot ROM knows how to boot our program
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// Set external crystal frequency
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

// Global state for the alarm, LED, and timer (similar to the working example)
type LedPin = Pin<hal::gpio::bank0::Gpio15, FunctionSio<SioOutput>, PullDown>;

// Global state for the alarm and LED (wrapped in Mutex for interrupt safety)
static G_ALARM: Mutex<RefCell<Option<Alarm0<CopyableTimer0>>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

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

    // Configure pin, get ownership of that pin
    let led_pin = pins.gpio15.into_push_pull_output();

    // Move ownership of TIMER0 peripheral to create Timer struct
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Create an alarm from the timer
    let mut alarm = timer.alarm_0().unwrap();

    // Set the alarm to trigger in 500 ms
    let _ = alarm.schedule(MicrosDurationU32::millis(500));

    // Enable alarm interrupt
    alarm.enable_interrupt();

    // Move alarm and LED to global state for interrupt handler
    critical_section::with(|cs| {
        G_ALARM.borrow(cs).replace(Some(alarm));
        G_LED.borrow(cs).replace(Some(led_pin));
    });

    // Enable the timer interrupt in NVIC
    unsafe {
        NVIC::unmask(hal::pac::Interrupt::TIMER0_IRQ_0);
    }

    // Main loop - do nothing
    loop {
        cortex_m::asm::wfi();
    }
}

// Timer interrupt handler
#[interrupt]
fn TIMER0_IRQ_0() {
    critical_section::with(|cs| {
        // Borrow the alarm and LED from global state
        let mut alarm_ref = G_ALARM.borrow(cs).borrow_mut();
        let mut led_ref = G_LED.borrow(cs).borrow_mut();

        if let (Some(alarm), Some(led)) = (alarm_ref.as_mut(), led_ref.as_mut()) {
            // Clear the interrupt
            alarm.clear_interrupt();

            // Toggle the LED
            let _ = led.toggle();

            // Schedule next interrupt in 500ms
            let _ = alarm.schedule(fugit::MicrosDurationU32::millis(500));
        }
    });
}
