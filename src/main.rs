extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::{thread, time};

fn main() {
    let mic = InputDevice::new(4);
    loop {
        println!("Value: {}", mic.value());
        thread::sleep(time::Duration::from_millis(50));
    }
}
