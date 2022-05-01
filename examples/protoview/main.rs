use std::fs::File;

use clap::Parser;

use libycresources::formats::pro;

pub(crate) mod print;

#[derive(Parser)]
#[clap(name = "protoview", version)]
struct Options {
    /// Path to the input proto file (.pro)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints out all available info about prototype
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

    let prototype = match pro::parse::prototype(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
            return;
        }
    };

    match options.action {
        Action::Dump => { print::prototype(&prototype) }
    }
}
