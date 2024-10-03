use rppal::gpio::{Gpio, Trigger};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let gpio = Gpio::new()?;

    let pin = gpio.get(21)?;
    let mut pin = pin.into_input();
    pin.set_interrupt(Trigger::Both, Some(Duration::from_millis(10)))?;

    loop {
        pin.poll_interrupt(true, None)?;
        println!("Event occurred");
    }
}
