pub(crate) mod extract;
pub(crate) mod platform;

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
    List,
    /// Extracts all archive contents to specified directory
    Extract(Extract),
}

#[derive(Clap)]
struct Extract {
    output: String,
}

fn main() {
    let options = Options::parse();

    let mut file = match File::open(&options.input) {
        Err(error) => {
            eprintln!("Couldn't open input file: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    let buffer_read_size: usize = 1 * 1024 * 1024;
    let mut reader = platform::reader::from(&mut file, buffer_read_size);

    if let Some(tree) = match dat::tree::read(&mut reader) {
        Err(error) => {
            eprintln!("Error occured: {:?}", error);
            return;
        }
        Ok(value) => value,
    } {
        match options.action {
            Action::List => {
                for (depth, is_last, directory) in tree.iter() {
                    if depth > 0 {
                        for _ in 0..depth - 1 {
                            print!("    ");
                        }

                        if is_last {
                            print!("└");
                        } else {
                            print!("├");
                        }
                        print!("───");
                    }

                    println!("{:}", directory.name);

                    for (index, file) in directory.files.iter().enumerate() {
                        let is_last_file = index == directory.files.iter().count() - 1
                            && directory.children.iter().count() == 0;

                        for _ in 0..depth {
                            print!("    ");
                        }

                        if is_last_file {
                            print!("└");
                        } else {
                            print!("├");
                        }
                        print!("───");
                        println!("{:}", file.name);
                    }
                }
            }
            Action::Extract(arguments) => {
                // let result = extract::entries(&mut reader, &entries, &arguments.output);
                // if let Err(error) = result {
                //     eprintln!("Error occured: {:?}", error);
                // }
            }
        }
    } else {
        println!("Input file has zero directories.");
    }
}
