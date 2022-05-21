use std::fs::File;

use clap::Parser;

use libycresources::formats::dat;

pub(crate) mod extract;
pub(crate) mod print;

#[derive(Parser)]
#[clap(name = "undat", version)]
struct Options {
    /// Path to the input archive file (.dat)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints archive contents
    Tree,
    /// Extracts all archive contents to specified directory
    Extract(Extract),
}

#[derive(Parser)]
struct Extract {
    output: String,
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

    if let Some(tree) = match dat::parse::tree(&mut reader) {
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
            return;
        }
        Ok(value) => value,
    } {
        match options.action {
            Action::Tree => {
                print::tree(&tree);
            }
            Action::Extract(arguments) => {
                let result = extract::tree(&mut reader, &tree, &arguments.output);

                if let Err(error) = result {
                    eprintln!("Error occurred: {:?}", error);
                }
            }
        }
    } else {
        println!("Input file has zero directories.");
    }
}
