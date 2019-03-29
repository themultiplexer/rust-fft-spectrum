extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;
use std::{thread, time};

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);
    
    loop {
        let value = block!(adc.read(&mut channel::SingleA0)).unwrap();
        println!("Value: {}", value);
        //thread::sleep(time::Duration::from_millis(50));
    }
    // get I2C device back
    let _dev = adc.destroy_ads1115();
}
