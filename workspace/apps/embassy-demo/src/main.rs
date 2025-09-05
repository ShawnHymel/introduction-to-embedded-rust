#![no_std]
#![no_main]

// Embassy: HAL imports
use embassy_rp::bind_interrupts;
use embassy_rp::gpio;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};

// Embassy: main executor
use embassy_executor::Spawner;

// Embassy: futures
use embassy_futures::join::join_array;

// Embassy: timer
use embassy_time::Timer;

// Embassy: sync
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

// Let panic_probe handle our panic routine
use panic_probe as _;

// Create signal handle
static SIGNAL_BLINK: Signal<CriticalSectionRawMutex, bool> = Signal::new();

// Macro to bind USB interrupt handler
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

// Task: handle USB logging
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// Handle button presses in main task
async fn monitor_button(mut pin: gpio::Input<'_>, id: &str) {
    let mut state = false;
    loop {
        // Yield waiting for button press
        pin.wait_for_low().await;
        log::info!("Button {} pressed", id);

        // Toggle and send signal to blinky thread
        state = !state;
        SIGNAL_BLINK.signal(state);

        // Simple debounce
        Timer::after_millis(200).await;
        pin.wait_for_high().await;
    }
}

// Task: blink the LED if the button is pressed
#[embassy_executor::task]
async fn blink_led_task(mut pin: gpio::Output<'static>) {
    let mut enabled = false;
    loop {
        // See if signal has anything, otherwise use previous `enabled` value
        enabled = SIGNAL_BLINK.try_take().unwrap_or(enabled);

        // Only blink if `enabled` is `true`
        if enabled {
            pin.set_high();
            Timer::after_millis(250).await;
        }
        pin.set_low();
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize embassy HAL
    let p = embassy_rp::init(Default::default());

    // Initialize USB driver and task
    let usb_driver = Driver::new(p.USB, Irqs);
    let _ = spawner.spawn(logger_task(usb_driver));

    // Create a new output pin
    let led_pin = gpio::Output::new(p.PIN_15, gpio::Level::Low);

    // Spawn blink task
    spawner.spawn(blink_led_task(led_pin)).unwrap();

    // Create a new input pin with an internal pulldown
    let btn_pin = gpio::Input::new(p.PIN_14, gpio::Pull::Up);

    // Create button monitor
    let btn_fut = monitor_button(btn_pin, "Pin 14");

    // Wait for the button monitor future to complete (it never will)
    join_array([btn_fut]).await;
}