
use std::time::{Duration, Instant};
use rumqttc::{Client, LastWill, MqttOptions, QoS};

use linux_embedded_hal::I2cdev;
use veml6030::{SlaveAddr, Veml6030};

fn threaded_read_lux_and_send_to_mqtt(_parm_lux:i32) {
    let dev = I2cdev::new("/dev/play/qwiic/i2c").unwrap();
    let address = SlaveAddr::Alternative(true);
    let mut sensor = Veml6030::new(dev, address);
    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    let mut mqttoptions = MqttOptions::new("    ", "sif.aaker.org", 1883);
    let will = LastWill::new("home office/ambient light", "ambient light sensor has gone offline.", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);

    let (client, _connection) = Client::new(mqttoptions, 10);
    sensor.enable().unwrap();
    loop {
        let local_lux = sensor.read_lux().unwrap();
        let payload = format!("{}", local_lux);

        println!("{}", payload);

        client.publish("home office/ambient light", QoS::AtLeastOnce, false, payload).unwrap();
        std::thread::sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn main() {
    let local_lux = 42;
    let read_lux_and_send_to_mqtt_thread = std::thread::spawn(move || threaded_read_lux_and_send_to_mqtt(local_lux));

    pretty_env_logger::init();

    let _local_result = read_lux_and_send_to_mqtt_thread.join();
}
