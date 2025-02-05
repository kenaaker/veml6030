# Rust VEML6030/VEML7700 High Accuracy Ambient Light Sensor Driver

[![crates.io](https://img.shields.io/crates/v/veml6030.svg)](https://crates.io/crates/veml6030)
[![Docs](https://docs.rs/veml6030/badge.svg)](https://docs.rs/veml6030)

This is an example of a Rust command for the VEML6030 high accuracy ambient 
light sensors using the [`embedded-hal`] traits.

This example allows you to:
- Enable/disable the device. See: `enable()`.
- Read the measured lux value. See: `read_lux()`.

## The devices

Vishay's VEML6030 is a high accuracy ambient light digital 16-bit
resolution sensors in a miniature transparent package. It includes
a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
and support an easy to use I2C bus communication interface and additional
interrupt feature.
The ambient light result is as digital value available.

Datasheets: [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf)

Application Notes:
- [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

```rust
use linux_embedded_hal::I2cdev;
use veml6030::{SlaveAddr, Veml6030};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Veml6030::new(dev, address);
    sensor.enable().unwrap();
    loop {
        let lux = sensor.read_lux().unwrap();
        println!("lux: {:2}", lux);
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
