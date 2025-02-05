use linux_embedded_hal::I2cdev;
use veml6030::{SlaveAddr, Veml6030};

fn main() {
    let dev = I2cdev::new("/dev/play/qwiic/i2c").unwrap();
    let address = SlaveAddr::Alternative(true);
    let mut sensor = Veml6030::new(dev, address);
    sensor.enable().unwrap();
    loop {
        let lux = sensor.read_lux().unwrap();
        println!("lux: {:2}", lux);
    }
}
