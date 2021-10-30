pub(crate) mod extract;
pub(crate) mod tree;

use libycresources::dat;

use clap::Clap;
use std::fs::File;

#[derive(Clap)]
#[clap(name = "undat", version)]
struct Options {
    /// Path to the input arhive file
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clap)]
enum Action {
    /// Prints arhive contents
    Tree,
    /// Extracts all archive contents to specified directory
    Extract(Extract),
}

#[derive(Clap)]
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
            eprintln!("Error occured: {:?}", error);
            return;
        }
        Ok(value) => value,
    } {
        match options.action {
            Action::Tree => {
                tree::print(&tree);
            }
            Action::Extract(arguments) => {
                let result = extract::tree(&mut reader, &tree, &arguments.output);

                if let Err(error) = result {
                    eprintln!("Error occured: {:?}", error);
                }
            }
        }
    } else {
        println!("Input file has zero directories.");
    }
}
