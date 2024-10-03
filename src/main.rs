use clap::Parser;
use futures::StreamExt;
use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};

#[derive(Debug, Parser)]
struct Arguments {
    /// The gpiochip device (e.g. /dev/gpiochip0)
    chip: String,

    /// The offset of the GPIO line for the provided chip
    line: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let mut chip = Chip::new(args.chip)?;
    let line = chip.get_line(args.line)?;

    let handle = line.events(
        LineRequestFlags::BIAS_PULL_UP,
        EventRequestFlags::BOTH_EDGES,
        "gpioevents",
    )?;
    let mut events = AsyncLineEventHandle::new(handle)?;

    while let Some(event) = events.next().await {
        match event {
            Ok(event) => println!("{event:?}"),
            Err(e) => eprintln!("{e:?}"),
        }
    }

    Ok(())
}
