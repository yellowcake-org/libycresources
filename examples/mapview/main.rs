use std::fs::File;

use clap::Parser;

use cli::{Action, Options};
use cli::export::filter::{Filter, Layers};
use libycresources::common::types::geometry::Scaled;
use libycresources::common::types::space::Elevation;
use libycresources::formats::map;
use provider::CommonProvider;

mod print;
mod render;
mod provider;
mod traits;
mod cli;

fn main() {
    let options = Options::parse();

    let directory = &options.resources.join("PROTO");
    let provider = CommonProvider { directory: directory.as_path() };

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
                .map_or(Layers::default(), |f| match f { Filter::Include(layers) => layers });

            let directory = &options.resources.join("ART");
            let provider = CommonProvider { directory: directory.as_path() };

            const MAX_ELEVATION: u8 = 2;
            let levels = 0..=MAX_ELEVATION;

            for level in levels {
                let level_readable = level + 1;
                let elevation = Elevation { level: Scaled { value: level, scale: 0..MAX_ELEVATION + 1 } };

                let result = render::map(
                    &map, &filter,
                    export.darkness.as_ref(),
                    &elevation,
                    &provider,
                    &options.resources,
                );

                let image = match result {
                    Ok(Some(value)) => {
                        println!("Succeeded rendering elevation {:?}.", level_readable);
                        value
                    }
                    Err(error) => {
                        eprintln!("Failed to render elevation {:?}. Error: {:?}", level_readable, error);
                        continue;
                    }
                    Ok(None) => {
                        println!("Elevation {:?} is not present in the file, skipping...", level_readable);
                        continue;
                    }
                };

                let filename = stem.to_str().map(|s|
                    s.to_string()).unwrap() + &format!("-{:?}", level_readable
                );

                let path = export.output.join(filename);
                let file = path.with_extension("bmp");

                if let Err(error) = image.save(file) {
                    return eprintln!("Couldn't save output file: {:}.", error);
                }
            }
        }
    }
}
