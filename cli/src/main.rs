use clap::Clap;
use five::emulator::Emulator;
use std::fs::File;
use std::io::Result;

#[derive(Clap)]
struct Opts {
    input: String,
}

fn main() -> Result<()> {
    let input = Opts::parse().input;
    let file = File::open(&input)?;
    let mut emulator = Emulator::default();
    emulator.load(file)?;
    let result = emulator.run();
    if result == 0 {
        println!("PASS: {}", input);
    } else {
        println!("FAIL({}): {}", result, input);
    }
    Ok(())
}
