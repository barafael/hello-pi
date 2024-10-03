use clap::Parser;
use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags};

#[derive(Debug, Parser)]
struct Arguments {
    /// The gpiochip device (e.g. /dev/gpiochip0)
    chip: String,

    /// The offset of the GPIO line for the provided chip
    line: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let mut chip = Chip::new(args.chip)?;
    let line = chip.get_line(args.line)?;

    for event in line.events(
        LineRequestFlags::BIAS_PULL_UP,
        EventRequestFlags::BOTH_EDGES,
        "gpioevents",
    )? {
        println!("{:?}", event?);
    }

    Ok(())
}
