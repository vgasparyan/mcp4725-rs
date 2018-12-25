//! A platform agnostic Rust driver for the MCP4725, based on the
//! [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! ## The Device
//!
//! The MCP4725 is a low-power, high accuracy, single channel,
//! 12-bit buffered voltage output Digital-to-Analog Convertor (DAC)
//! with non-volatile memory (EEPROM).
//!
//! Datasheet: http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf
//!

#![no_std]
#![deny(missing_docs)]

extern crate embedded_hal as hal;

use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write, WriteRead};

/// Driver for the MCP4725 DAC
#[derive(Debug, Default)]
pub struct MCP4725<I2C, D> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    /// The I2C device address.
    address: u8,
    /// The concrete Delay implementation.
    delay: D,
}

impl<I2C, D, E> MCP4725<I2C, D>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u8>,
{
    /// Initialize the MCP4725 driver.
    pub fn new(i2c: I2C, address: u8, delay: D) -> Result<Self, E> {
        let mcp4725 = MCP4725 {
            i2c,
            address,
            delay,
        };
        Ok(mcp4725)
    }

    ///write dac register, fast mode
    pub fn write_dac_register_fast(&mut self, input_code: u16) -> Result<(), E> {
        let b1 = (input_code >> 8) as u8;
        let b2 = (input_code & 0xFF) as u8;
        self.i2c.write(self.address, &[b1, b2])?;
        Ok(())
    }

    ///write dac register
    pub fn write_dac_register(&mut self, input_code: u16) -> Result<(), E> {
        let b1 = (input_code >> 4) as u8;
        let b2 = ((input_code << 4) & 0xFF) as u8;
        self.i2c.write(self.address, &[0x40, b1, b2])?;
        Ok(())
    }

    ///write dac register and eeprom
    pub fn write_dac_register_eeprom(&mut self, input_code: u16) -> Result<(), E> {
        let b1 = (input_code >> 4) as u8;
        let b2 = ((input_code << 4) & 0xFF) as u8;
        self.i2c.write(self.address, &[0x60, b1, b2])?;
        Ok(())
    }

    ///read DAC value and return as 16-bit value
    pub fn read_dac_register(&mut self) -> Result<u16, E> {
        let mut buf = [0, 0, 0];
        //read 3-bytes from device, last two bytes are DAC value
        self.i2c.read(self.address, &mut buf)?;
        let val = ((buf[1] as u16) << 4) | (buf[2] >> 4) as u16;
        Ok(val)
    }

    ///set DAC output to specified value using fast mode.
    ///Value range is [0-4095], which corresponds to [0-VDD] voltage
    pub fn set_dac_value(&mut self, val: u16) -> Result<(), E> {
        let dac_code = val & 0xFFF;
        self.write_dac_register_fast(dac_code)?;
        Ok(())
    }

    ///get current value of the DAC output
    pub fn get_dac_value(&mut self) -> Result<u16, E> {
        let v = self.read_dac_register()?;
        Ok(v)
    }

    ///performs internal reset, after this event, the device uploads
    ///the contents of the EEPROM into the DAC register
    pub fn reset(&mut self) -> Result<(), E> {
        self.i2c.write(self.address, &[0x06])?;
        Ok(())
    }
}
