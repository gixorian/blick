use std::{error::Error, process};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = list::Cli::parse();

    if let Err(e) = list::run(args) {
        print!("{e}");
        process::exit(1);
    };

    Ok(())
}
