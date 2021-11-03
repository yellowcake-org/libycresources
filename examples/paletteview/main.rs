pub mod render;

use libycresources::common::graphics;
use libycresources::formats::pal;

use clap::Parser;
use std::fs::File;

#[derive(Parser)]
#[clap(name = "paletteview", version)]
struct Options {
    /// Path to the input palette file (.pal)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints out summary about palette
    Info,
    /// Renders palette into .bmp file in specified directory
    Render(Render),
}

#[derive(Parser)]
struct Render {
    directory: String,
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

    let palette = match pal::parse::palette(&mut reader) {
        Err(error) => {
            eprintln!("Error occured: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    match options.action {
        Action::Info => {
            let flatten_colors: Vec<graphics::ColorPixel> =
                palette.colors.into_iter().flatten().collect();

            println!("Palette contains {:} valid colors.", flatten_colors.len());
        }
        Action::Render(arguments) => {
            let output = std::path::Path::new(&arguments.directory);

            if !output.exists() {
                eprintln!("Output path does not exist. Aborting.");
                return;
            }

            if !output.is_dir() {
                eprintln!("Output path is not a directory. Aborting.");
                return;
            }

            let filename = match std::path::Path::new(&options.input).file_stem() {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't determine palette output filename.");
                    return;
                }
            };

            let path = output.join(filename).with_extension("bmp");

            match render::image(&palette.colors, 8).save(path) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Couldn't write output file: {:}", error);
                    return;
                }
            }
        }
    }
}
