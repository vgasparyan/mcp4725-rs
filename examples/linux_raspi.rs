extern crate linux_embedded_hal as linux_hal;
extern crate mcp4725_rs;

use linux_hal::{Delay, I2cdev};
use mcp4725_rs::MCP4725;

fn main() {
    println!("Hello, MCP4725!");
    println!();
    println!("Generating sawtooth wave signal on DAC output...");

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x60; //default slave address

    let mut dac = MCP4725::new(dev, address, Delay).unwrap();
    //generate sawtooth wave signal with amplitude from 0 to VDD
    loop {
        for dac_code in 0..4096 {
            dac.set_dac_value(dac_code);
        }
    }
}
