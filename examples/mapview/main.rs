use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use libycresources::formats::map;

use crate::provider::Provider;

mod print;
mod export;
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
    /// Action to perform on provided map file
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    /// Prints out all available info about map
    Dump,
    /// Renders the map into .bmp file
    Export(Layers),
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
        Action::Export(layers) => { export::export(&map, &layers) }
    }
}
