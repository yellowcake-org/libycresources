use std::fs::File;

use clap::Parser;

use ycresources::formats::aaf;

pub(crate) mod print;

#[derive(Parser)]
#[clap(name = "fontview", version)]
struct Options {
    /// Path to the input font file (.aaf)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints all glyphs from specified font
    Dump,
    /// Prints specified string with glyphs from font
    Print(Print),
}

#[derive(Parser)]
struct Print {
    string: String,
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

    let font = match aaf::parse::font(&mut reader) {
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    match options.action {
        Action::Dump => {
            println!("Line height: {:?}", font.height);
            println!("Vertical spacing: {:?}", font.spacing.vertical);
            println!("Horizontal spacing: {:?}", font.spacing.horizontal);

            println!();

            for glyph in font.glyphs {
                print::glyph(&glyph);
            }
        }
        Action::Print(arguments) => {
            for char in arguments.string.chars() {
                if char.is_ascii() {
                    if let Some(glyph) = &font.glyphs.get(char as usize) {
                        print::glyph(&glyph);
                    } else {
                        eprintln!("Encountered char, which is absent in provided font. Aborting.");
                        return;
                    }
                } else {
                    eprintln!("Non-ASCII char was found within provided string. Aborting.");
                    return;
                }
            }
        }
    }
}
