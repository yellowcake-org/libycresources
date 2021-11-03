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

            let width = 8;
            let height = palette_regular.colors.len() / width;

            let palette_pixels = palette_regular.colors.map(|color| match color {
                None => bmp::Pixel::new(0, 0, 0),
                Some(color) => {
                    let red = ((color.red.value * std::u8::MAX as usize)
                        / ((color.red.scale.end - color.red.scale.start) as usize))
                        as u8;
                    let green = ((color.green.value * std::u8::MAX as usize)
                        / ((color.green.scale.end - color.green.scale.start) as usize))
                        as u8;
                    let blue = ((color.blue.value * std::u8::MAX as usize)
                        / ((color.blue.scale.end - color.blue.scale.start) as usize))
                        as u8;

                    bmp::Pixel::new(red, green, blue)
                }
            });

            let mut palette_image = bmp::Image::new(width as u32, height as u32);

            for (x, y) in palette_image.coordinates() {
                palette_image.set_pixel(x, y, palette_pixels[(width as u32 * y + x) as usize]);
            }

            match palette_image.save(output.join("palette.bmp")) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Couldn't write palette.bmp: {:}", error);
                    return;
                }
            }
        }
    }
}
