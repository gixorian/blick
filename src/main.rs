use std::{error::Error, process};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = blick::Cli::parse();

    if let Err(e) = blick::run(args) {
        print!("{e}");
        process::exit(1);
    };

    Ok(())
}
