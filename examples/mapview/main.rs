use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use libycresources::formats::map;

use crate::provider::Provider;

mod print;
mod render;
mod provider;
mod traits;

#[derive(Parser)]
#[clap(name = "mapview", version)]
struct Options {
    /// Path to the input map file (.map)
    #[clap(short, long)]
    input: PathBuf,
    /// Path to the root resources directory
    #[clap(short, long)]
    resources: PathBuf,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints out all available info about map
    Dump,
    /// Renders the map into .bmp file
    Export(Export),
}

#[derive(Parser)]
struct Export {
    /// Path to the output image file (.bmp)
    #[clap(short, long)]
    output: PathBuf,
    #[clap(subcommand)]
    filter: Option<Filter>,
}

#[derive(Parser)]
enum Filter {
    /// Optional filter for which layers to render
    Include(Layers)
}

#[derive(Parser)]
pub(crate) struct Layers {
    #[clap(short, long)]
    floor: bool,
    #[clap(short, long)]
    roof: bool,
    #[clap(short, long)]
    walls: bool,
    #[clap(short, long)]
    scenery: bool,
}

fn main() {
    let options = Options::parse();

    let directory = &options.resources.join("PROTO");
    let provider = Provider { directory: directory.as_path() };

    let map = match File::open(&options.input) {
        Err(error) => { return eprintln!("Couldn't open input file: {:?}", error); }
        Ok(value) => value,
    };

    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, map);
    let map = match map::parse::map(&mut reader, &provider) {
        Err(error) => { return eprintln!("Couldn't parse map file: {:?}", error); }
        Ok(value) => value,
    };

    match options.action {
        Action::Dump => { print::map(&map) }
        Action::Export(export) => {
            if !export.output.exists() { return eprintln!("Output path does not exist. Aborting."); }
            if !export.output.is_dir() { return eprintln!("Output path is not a directory. Aborting."); }

            let stem = match options.input.file_stem() {
                None => { return eprintln!("Couldn't determine frame output filename."); }
                Some(value) => value,
            };

            let filter = export.filter
                .map_or(
                    Layers { floor: false, roof: false, walls: false, scenery: false },
                    |f| { match f { Filter::Include(layers) => layers } },
                );

            let directory = &options.resources.join("ART");
            let provider = Provider { directory: directory.as_path() };

            let image = match render::map(&map, &filter, &provider, &options.resources) {
                Err(error) => { return eprintln!("Couldn't render map file: {:}", error); }
                Ok(value) => value,
            };

            let path = export.output.join(stem);
            let file = path.with_extension("bmp");

            if let Err(error) = image.save(file) {
                return eprintln!("Couldn't save output file: {:}", error);
            }
        }
    }
}
