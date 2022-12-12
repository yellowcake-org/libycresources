use std::path::PathBuf;

use clap::Parser;

use darkness::Darkness;
use elevation::Elevation;
use filter::Filter;

pub(crate) mod filter;
pub(crate) mod darkness;
pub(crate) mod elevation;

#[derive(Parser)]
pub(crate) struct Export {
    /// Path to the output image file
    #[clap(short, long)]
    pub(crate) output: PathBuf,

    /// Custom darkness to use instead of the value in the map
    #[clap(arg_enum)]
    #[clap(short, long)]
    pub(crate) darkness: Option<Darkness>,

    /// Render specified elevation only
    #[clap(arg_enum)]
    #[clap(short, long)]
    pub(crate) elevation: Option<Elevation>,

    #[clap(subcommand)]
    pub(crate) filter: Option<Filter>,
}
