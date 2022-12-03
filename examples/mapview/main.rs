use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use libycresources::formats::map;

use crate::provider::Provider;

mod print;
mod render;
mod provider;

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
    background: bool,
    #[clap(short, long)]
    tiles: bool,
    #[clap(short, long)]
    walls: bool,
    #[clap(short, long)]
    scenery: bool,
}

fn main() {
    let options = Options::parse();

    let protos = &options.resources.join("PROTO");
    let arts = &options.resources.join("ART");

    let protos = Provider { directory: protos.as_path() };
    let arts = Provider { directory: arts.as_path() };

    let file = match File::open(&options.input) {
        Ok(value) => value,
        Err(error) => { return eprintln!("Couldn't open input file: {:?}", error); }
    };

    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, file);
    let map = match map::parse::map(&mut reader, &protos) {
        Ok(value) => value,
        Err(error) => { return eprintln!("Couldn't parse map file: {:?}", error); }
    };

    match options.action {
        Action::Dump => { print::map(&map) }
        Action::Export(export) => {
            let filter = export.filter
                .map_or(
                    Layers { background: false, tiles: false, walls: false, scenery: false },
                    |f| { match f { Filter::Include(layers) => layers } },
                );

            if !export.output.exists() {
                eprintln!("Output path does not exist. Aborting.");
                return;
            }

            if !export.output.is_dir() {
                eprintln!("Output path is not a directory. Aborting.");
                return;
            }

            let stem = match options.input.file_stem() {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't determine frame output filename.");
                    return;
                }
            };

            let path = export.output.join(stem);
            let file = path.with_extension("bmp");

            if let Err(error) = render::map(&map, &filter).save(file) {
                eprintln!("Couldn't write output file: {:}", error);
                return;
            }
        }
    }
}
