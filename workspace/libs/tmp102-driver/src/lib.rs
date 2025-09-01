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

#[cfg(test)]
mod tests {

    // Import top-level structs/functions
    use super::*;

    // Explicitly link to std
    extern crate std;

    // Test-only imports
    use embedded_hal::i2c::{Error as I2cError, ErrorKind, Operation};

    // I2C stub
    #[derive(Debug)]
    pub struct I2cStub {
        pub response_data: [u8; 2],
        pub call_count: usize,
    }

    // I2C bus with temperature sensor stub implementation
    impl I2cStub {

        // Create a new I2C bus
        pub fn new() -> Self {
            Self {
                response_data: [0x00, 0x00],
                call_count: 0,
            }
        }

        // Set temperature (in Celsius)
        pub fn set_temperature(&mut self, temp_c: f32) {
            // Convert temperature to sensor format
            let temp_raw = ((temp_c / 0.0625) as i16) << 4;
            self.response_data[0] = (temp_raw >> 8) as u8;
            self.response_data[1] = (temp_raw & 0xFF) as u8;
        }
    }

    // Declare a dummy error type
    #[derive(Debug, Clone)]
    pub struct DummyError;

    // Implement I2cError trait for the DummyError type
    impl I2cError for DummyError {
       fn kind(&self) -> ErrorKind {
        ErrorKind::Other
       }
    }

    // Associated type: use our dummy error type (for e.g. Self::Error)
    impl embedded_hal::i2c::ErrorType for I2cStub {
        type Error = DummyError;
    }

    // Stub mplementations of the basic I2C read/write functions
    impl embedded_hal::i2c::I2c for I2cStub {

        // Always return Ok
        fn read(
            &mut self, 
            _address: u8, 
            _read: &mut [u8]
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        // Always return Ok
        fn write(
            &mut self, 
            _address: u8, 
            _write: &[u8]
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        // Always return Ok
        fn write_read(
            &mut self,
            _address: u8,
            _write: &[u8],
            read: &mut [u8],
        ) -> Result<(), Self::Error> {
            // Return canned response
            read.copy_from_slice(&self.response_data);
            self.call_count += 1;
            Ok(())
        }

        // Always return Ok
        fn transaction(
            &mut self,
            _address: u8,
            _operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    // Unit test 1: create a new driver and make sure the device address is set
    #[test]
    fn test_new_driver() {
        let i2c = I2cStub::new();
        let driver = TMP102::new(i2c, Address::Ground);
        assert_eq!(driver.address.as_u8(), 0x48);
    }

    // Unit test 2: Read the temperature using the I2C stub driver
    #[test]
    fn test_temperature_read() {
        // Create a new I2C stub driver
        let mut i2c = I2cStub::new();

        // Set the temperature
        i2c.set_temperature(25.0);

        // Read the temperature
        let mut driver = TMP102::new(i2c, Address::Ground);
        let temp = driver.read_temperature_c().unwrap();

        assert_eq!(temp, 25.0);
    }
}