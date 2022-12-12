use std::path::PathBuf;

use clap::Parser;

use darkness::Darkness;
use filter::Filter;

pub mod filter;
pub mod darkness;

#[derive(Parser)]
pub(crate) struct Export {
    /// Path to the output image file (.bmp)
    #[clap(short, long)]
    pub(crate) output: PathBuf,
    /// Custom darkness to use instead of the value in the map
    #[clap(arg_enum)]
    #[clap(short, long)]
    pub(crate) darkness: Option<Darkness>,
    #[clap(subcommand)]
    pub(crate) filter: Option<Filter>,
}
