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

## Examples
Build the example on Raspberry PI

cargo build --examples

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.
