use std::fs::File;

use clap::Parser;

use libycresources::formats::{frm, pal};

pub mod render;

#[derive(Parser)]
#[clap(name = "frameview", version)]
struct Options {
    /// Path to the input frame file (.frm, .fr0-5)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints summary info about contents of the file
    Info,
    /// Renders frame contents into .bmp file(s) in specified directory
    Render(Render),
}

#[derive(Parser)]
struct Render {
    /// Path to the input palette file (.pal)
    #[clap(short, long)]
    palette: String,
    /// Output directory path
    #[clap(short, long)]
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

    let sprite = match frm::parse::sprite(&mut reader) {
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    match options.action {
        Action::Info => {
            println!("FPS: {:}.", sprite.fps);
            println!("Frames per orientation: {:}.", sprite.count);
            println!("Unique animations: {:}.", sprite.animations.len());
            println!("Orientated animations: {:?}.", sprite.orientations);

            println!();

            for (index, animation) in sprite.animations.iter().enumerate() {
                println!(
                    "Animation {:} has {:} frame(s).",
                    index,
                    animation.frames.len()
                );
            }
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

            let directory_name = match std::path::Path::new(&options.input).file_stem() {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't determine frame output filename.");
                    return;
                }
            };

            let path = output.join(directory_name);

            let mut reader = std::io::BufReader::with_capacity(
                1 * 1024 * 1024,
                match File::open(arguments.palette) {
                    Err(error) => {
                        eprintln!("Couldn't open palette file: {:?}", error);
                        return;
                    }
                    Ok(value) => value,
                },
            );

            let palette = match pal::parse::palette(&mut reader) {
                Err(error) => {
                    eprintln!("Error occurred: {:?}", error);
                    return;
                }
                Ok(value) => value,
            };

            for (index, animation) in sprite.animations.iter().enumerate() {
                let animation_dir = path.join(index.to_string());

                match std::fs::create_dir_all(&animation_dir) {
                    Err(error) => {
                        eprintln!(
                            "Couldn't create animation output directory tree: {:?}, error: {:}",
                            animation_dir, error
                        );
                        return;
                    }
                    Ok(_) => (),
                };

                for (index, frame) in animation.frames.iter().enumerate() {
                    let frame_path = animation_dir.join(index.to_string()).with_extension("bmp");

                    match render::frame(&frame, &palette) {
                        Ok(image) => {
                            match image.save(frame_path) {
                                Ok(_) => {}
                                Err(error) => {
                                    eprintln!("Couldn't write output file: {:}", error);
                                    return;
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("Couldn't write output file: {:?}", error);
                            return;
                        }
                    }
                }
            }
        }
    }
}
