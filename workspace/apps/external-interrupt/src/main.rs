#![no_std]
#![no_main]

// We need to write our own panic handler
use core::panic::PanicInfo;

// Let us modify data with only immutable reference (enforce borrow rules at runtime)
use core::cell::RefCell;

// Atomic operations for setting a global flag
use core::sync::atomic::{AtomicBool, Ordering};

// Embedded mutex (no threads): access to data by one piece of code at a time
use critical_section::Mutex;

// Alias our HAL
use rp235x_hal as hal;

// Import traits for embedded abstractions
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, StatefulOutputPin};

// Imports for the pin definition
use hal::gpio::{FunctionSio, Interrupt, Pin, PullUp, SioInput};

// Imports for the timer interrupt
use hal::pac::interrupt;

// Direct access to the nested vectored interrupt controller (NVIC)
use cortex_m::peripheral::NVIC;

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
type ButtonPin = Pin<hal::gpio::bank0::Gpio14, FunctionSio<SioInput>, PullUp>;

// Global state for the button (wrapped in Mutex for interrupt safety)
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));

// Global flag for button press
static G_BTN_FLAG: AtomicBool = AtomicBool::new(false);

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

    // Move ownership of TIMER0 peripheral to create Timer struct
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Configure pin, get ownership of that pin
    let mut led_pin = pins.gpio15.into_push_pull_output();
    let btn_pin = pins.gpio14.into_pull_up_input();

    // Trigger on falling edge (button press)
    btn_pin.set_interrupt_enabled(Interrupt::EdgeLow, true);

    // Move alarm and LED to global state for interrupt handler
    critical_section::with(|cs| {
        G_BUTTON.borrow(cs).replace(Some(btn_pin));
    });

    // Enable the interrupt line
    unsafe {
        NVIC::unmask(hal::pac::Interrupt::IO_IRQ_BANK0);
    }

    // Main loop
    loop {
        // Wait for interrupt
        cortex_m::asm::wfi();

        // Check if the button was pressed (non-blocking)
        if G_BTN_FLAG.load(Ordering::Acquire) {
            // Clear the flag
            G_BTN_FLAG.store(false, Ordering::Release);

            // Simple debounce: block for 50 ms then check button state
            timer.delay_ms(50);
            critical_section::with(|cs| {
                let mut btn_ref = G_BUTTON.borrow(cs).borrow_mut();
                if let Some(button) = btn_ref.as_mut() {
                    if button.is_low().unwrap_or(false) {
                        // Toggle LED
                        let _ = led_pin.toggle();
                    }
                }
            });
        }
    }
}

// Interrupt service routine (ISR)
#[interrupt]
fn IO_IRQ_BANK0() {
    critical_section::with(|cs| {
        // Borrow the button from global state
        let mut btn_ref = G_BUTTON.borrow(cs).borrow_mut();

        // Get mutable references
        if let Some(button) = btn_ref.as_mut() {
            // Check if the interrupt source was the pin going from high to low
            if button.interrupt_status(Interrupt::EdgeLow) {
                // Clear the interrupt
                button.clear_interrupt(Interrupt::EdgeLow);

                // Set the flag
                G_BTN_FLAG.store(true, Ordering::Release);
            }
        }
    });
}
