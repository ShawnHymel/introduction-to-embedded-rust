#![no_std]
#![no_main]

// Embassy imports
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio;
use embassy_time::Timer;

// ???
use gpio::{Level, Output};

// Debugging output
use defmt::*;
use defmt_rtt as _;

// Let panic_probe handle our panic routine
use panic_probe as _;

// Copy boot metadata to .start_block so Boot ROM knows how to boot our program
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);

    loop {
        info!("led on!");
        led.set_high();
        Timer::after_millis(250).await;

        info!("led off!");
        led.set_low();
        Timer::after_millis(250).await;
    }
}