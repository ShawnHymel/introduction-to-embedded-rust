// #![no_std]
// #![no_main]

// // Embassy imports
// use embassy_executor::Spawner;
// use embassy_rp::block::ImageDef;
// use embassy_rp::gpio;
// use embassy_rp::peripherals::USB;
// use embassy_rp::usb::Driver;
// use embassy_time::Timer;

// // ???
// use gpio::{Level, Output};

// // Debugging output
// use defmt::*;
// use defmt_rtt as _;

// // Let panic_probe handle our panic routine
// use panic_probe as _;

// // Copy boot metadata to .start_block so Boot ROM knows how to boot our program
// #[unsafe(link_section = ".start_block")]
// #[used]
// pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let p = embassy_rp::init(Default::default());
//     let mut led = Output::new(p.PIN_15, Level::Low);

//     loop {
//         debug!("led on!");
//         led.set_high();
//         Timer::after_millis(250).await;

//         debug!("led off!");
//         led.set_low();
//         Timer::after_millis(250).await;
//     }
// }

// #[embassy_executor::task]
// async fn logger_task(driver: Driver<'static, USB>) {
//    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
// }

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);
    let _ = spawner.spawn(logger_task(driver));

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);
        Timer::after_secs(1).await;
    }
}