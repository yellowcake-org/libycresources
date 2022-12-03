use std::fs::File;
use std::path::PathBuf;

use libycresources::common::types::errors::Error;
use libycresources::formats::pal;

pub(crate) fn palette(path: &PathBuf) -> Result<pal::Palette, Error> {
    let file = File::open(path)?;
    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, file);

    pal::parse::palette(&mut reader)
}