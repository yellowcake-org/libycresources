use std::fs::File;

use clap::Parser;

use libycresources::formats::map;

pub(crate) mod print;

#[derive(Parser)]
#[clap(name = "mapview", version)]
struct Options {
    /// Path to the input map file (.map)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints out all available info about map
    Dump,
}

fn main() {
    let options = Options::parse();

    let file = match File::open(&options.input) {
        Err(error) => {
            eprintln!("Couldn't open input file: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, file);

    let map = match map::parse::map(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
            return;
        }
    };

    match options.action {
        Action::Dump => { print::map(&map) }
    }
}
