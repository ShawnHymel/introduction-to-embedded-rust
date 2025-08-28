#![no_std]

//! # TMP102 Demo Driver
//!
//! A simple demo driver for the TMP102 temperature sensor

use embedded_hal::i2c::I2c;

/// Custom error for our crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C communication error
    Communication(E),
}

/// Possible device addresses based on ADD0 pin connection
#[derive(Debug, Clone, Copy)]
pub enum Address {
    Ground = 0x48, // Default
    Vdd = 0x49,
    Sda = 0x4A,
    Scl = 0x4B,
}

impl Address {
    /// Get the I2C address in u8 format
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// List internal registers in a struct
#[allow(dead_code)]
struct Register;

#[allow(dead_code)]
impl Register {
    const TEMPERATURE: u8 = 0x00;
    const CONFIG: u8 = 0x01;
    const T_LOW: u8 = 0x02;
    const T_HIGH: u8 = 0x03;
}

/// TMP102 temperature sensor driver
pub struct TMP102<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C> TMP102<I2C>
where
    I2C: I2c,
{
    /// Create a new TMP102 driver instance
    pub fn new(i2c: I2C, address: Address) -> Self {
        Self { i2c, address }
    }

    /// Create new instance with default address (Ground)
    pub fn with_default_address(i2c: I2C) -> Self {
        Self::new(i2c, Address::Ground)
    }

    /// Read the current temperature in degrees Celsius (blocking)
    pub fn read_temperature_c(&mut self) -> Result<f32, Error<I2C::Error>> {
        let mut rx_buf = [0u8; 2];

        // Read from sensor
        match self
            .i2c
            .write_read(self.address.as_u8(), &[Register::TEMPERATURE], &mut rx_buf)
        {
            Ok(()) => Ok(self.raw_to_celsius(rx_buf)),
            Err(e) => Err(Error::Communication(e)),
        }
    }

    /// Convert raw reading to Celsius
    fn raw_to_celsius(&self, buf: [u8; 2]) -> f32 {
        let temp_raw = ((buf[0] as u16) << 8) | (buf[1] as u16);
        let temp_signed = (temp_raw as i16) >> 4;
        (temp_signed as f32) * 0.0625
    }
}
