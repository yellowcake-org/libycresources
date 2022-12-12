use std::path::PathBuf;

use clap::Parser;

use export::Export;

pub(crate) mod export;

#[derive(Parser)]
#[clap(name = "mapview", version)]
pub(crate) struct Options {
    /// Path to the input map file (.map)
    #[clap(short, long)]
    pub(crate) input: PathBuf,
    /// Path to the root resources directory
    #[clap(short, long)]
    pub(crate) resources: PathBuf,
    #[clap(subcommand)]
    pub(crate) action: Action,
}

#[derive(Parser)]
pub(crate) enum Action {
    /// Prints out all available info about map
    Dump,
    /// Renders the map into .bmp file
    Export(Export),
}
