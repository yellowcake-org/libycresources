use std::fs::File;

use clap::Parser;

use libycresources::formats::pro;

#[derive(Parser)]
#[clap(name = "fontview", version)]
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
        Action::Dump => {
            println!("Common fields:");
            println!();
            println!("Object ID: {:?}", prototype.id);
            println!("Text ID: {:?}", prototype.meta.connections.description_id);
            println!("Sprite ID: {:?}", prototype.meta.sprite.id);
            println!("Sprite type: {:?}", prototype.meta.sprite.r#type);
            println!("Light radius: {:?}", prototype.meta.light.distance);
            println!("Light intensity: {:?}", prototype.meta.light.intensity);
            println!("Flags: {:?}", prototype.meta.flags);
        }
    }
}
