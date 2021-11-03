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
    /// Prints out summary about regular and animated palettes
    Info,
    /// Renders regular and animated palettes as .bmp files into specified directory
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

    let values = match pal::parse::values(&mut reader) {
        Err(error) => {
            eprintln!("Error occured: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    let palette_regular = pal::palette::calculate::regular(&values);
    let palette_animated = pal::palette::calculate::animated(&values);

    match options.action {
        Action::Info => {
            let flatten_colors: Vec<graphics::ColorPixel> =
                palette_regular.colors.into_iter().flatten().collect();

            println!(
                "Regular palette has {:} valid colors.",
                flatten_colors.len()
            );

            println!(
                "Alarm animation has {:} colors at {:?} duration for each.",
                palette_animated.alarm.values.len(),
                palette_animated.alarm.duration
            );

            println!(
                "Slime animation has {:} colors at {:?} duration for each.",
                palette_animated.slime.values.len(),
                palette_animated.slime.duration
            );

            println!(
                "Shore animation has {:} colors at {:?} duration for each.",
                palette_animated.shore.values.len(),
                palette_animated.shore.duration
            );

            println!(
                "Screen animation has {:} colors at {:?} duration for each.",
                palette_animated.screen.values.len(),
                palette_animated.screen.duration
            );

            println!(
                "Slow fire animation has {:} colors at {:?} duration for each.",
                palette_animated.fire_slow.values.len(),
                palette_animated.fire_slow.duration
            );

            println!(
                "Fast fire animation has {:} colors at {:?} duration for each.",
                palette_animated.fire_fast.values.len(),
                palette_animated.fire_fast.duration
            );
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

            {
                let path = output.join(filename).with_extension("bmp");

                match render::image(&palette_regular.colors, 8).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write palette bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("alarm")
                    .with_extension("bmp");

                let alarm: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .alarm
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(alarm.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write alarm bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("slime")
                    .with_extension("bmp");

                let slime: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .slime
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(slime.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write slime bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("shore")
                    .with_extension("bmp");

                let shore: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .shore
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(shore.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write shore bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("screen")
                    .with_extension("bmp");

                let screen: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .screen
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(screen.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write screen bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("fire_slow")
                    .with_extension("bmp");

                let fire_slow: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .fire_slow
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(fire_slow.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write fire_slow bitmap: {:}", error);
                        return;
                    }
                }
            }

            {
                let path = output
                    .join(filename)
                    .with_file_name("fire_fast")
                    .with_extension("bmp");

                let fire_fast: Vec<Option<graphics::ColorPixel>> = palette_animated
                    .fire_fast
                    .values
                    .into_iter()
                    .map(|v| Some(v))
                    .collect();

                match render::image(fire_fast.as_slice(), 1).save(path) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Couldn't write fire_fast bitmap: {:}", error);
                        return;
                    }
                }
            }
        }
    }
}
