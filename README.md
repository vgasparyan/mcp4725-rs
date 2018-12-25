# `Rust MC4725 Driver`

> This is a platform agnostic Rust driver for the MCP4725, based on the
[`embedded-hal`](https://github.com/japaric/embedded-hal) traits.


## The Device

The Microchip MCP4725 is a low-power, high accuracy, single channel,
12-bit buffered voltage output Digital-to-Analog Convertor (DAC)
with non-volatile memory (EEPROM).

Datasheet: http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf

## Status

- [x] Support generation of DAC output using Fast Mode Write Command
- [x] Support writing DAC Input register and EEPROM
- [x] Support RESET command
- [x] Test on Raspberry Pi

## [Examples](examples)

From [`examples/linux_raspi.rs`](examples/linux_raspi.rs):

```rust
extern crate linux_embedded_hal as linux_hal;
extern crate mcp4725_rs;

use linux_hal::{Delay, I2cdev};
use mcp4725_rs::MCP4725;

fn main() {
    println!("Hello, MCP4725!");
    println!();
    println!("Generating sawtooth wave signal on DAC output...");

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x60;

    let mut dac = MCP4725::new(dev, address, Delay).unwrap();
    
    loop {
        for dac_code in 0..4096 {
            dac.set_dac_value(dac_code);
        }
    }
}
```

Build the example on Raspberry PI

cargo build --examples

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.
