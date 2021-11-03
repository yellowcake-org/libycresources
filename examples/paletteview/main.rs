use libycresources::common::graphics::*;
use libycresources::formats::pal;

use clap::Clap;
use std::fs::File;

#[derive(Clap)]
#[clap(name = "paletteview", version)]
struct Options {
    /// Path to the input palette file (.pal)
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clap)]
enum Action {
    /// Prints out summary about regular and animated palettes
    Info,
    /// Renders regular and animated palettes as .bmp files into specified directory
    Render(Render),
}

#[derive(Clap)]
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
            let flatten_colors: Vec<ColorPixel> =
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
            //
        }
    }
}
