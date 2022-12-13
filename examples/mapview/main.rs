use std::fs::File;

use clap::Parser;

use cli::{Action, Options};
use cli::export::elevation;
use cli::export::filter::{Filter, Layers};
use libycresources::common::types::geometry::Scaled;
use libycresources::common::types::space;
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
        Err(error) => { return eprintln!("Couldn't open input file: {:?}.", error); }
        Ok(value) => value,
    };

    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, map);
    let map = match map::parse::map(&mut reader, &provider) {
        Err(error) => { return eprintln!("Couldn't parse map file: {:?}.", error); }
        Ok(value) => value,
    };

    match options.action {
        Action::Dump => { print::map(&map) }
        Action::Export(export) => {
            if !export.output.exists() { return eprintln!("Output path does not exist. Aborting."); }
            if !export.output.is_dir() { return eprintln!("Output path is not a directory. Aborting."); }

            let stem = match options.input.file_stem() {
                None => { return eprintln!("Couldn't determine output filename. Aborting."); }
                Some(value) => value,
            };

            let filter = export.filter
                .map_or(Layers::default(), |f| match f { Filter::Layers(layers) => layers });

            let directory = &options.resources.join("ART");
            let provider = CommonProvider { directory: directory.as_path() };

            const MAX_ELEVATION: u8 = 2;
            let levels = export.elevation.as_ref()
                .map_or(0..=MAX_ELEVATION, |e| {
                    let level = match e {
                        elevation::Elevation::First => 0,
                        elevation::Elevation::Second => 1,
                        elevation::Elevation::Third => 2
                    } as u8;

                    level..=level
                });

            if !filter.all() { println!("Layers filter has been applied."); }
            if export.darkness.as_ref().is_some() { println!("Darkness customization has been applied."); }
            if export.elevation.as_ref().is_some() { println!("Provided elevation will be rendered only."); }

            for level in levels {
                let level_readable = level + 1;
                let elevation = space::Elevation { level: Scaled { value: level, scale: 0..MAX_ELEVATION + 1 } };

                println!("Started rendering level {:?}...", level_readable);
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
                        eprintln!("Failed to render elevation {:?}. Error: {:?}.", level_readable, error);
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

                println!("Writing to file...");
                if let Err(error) = image.save(file) {
                    eprintln!("Couldn't save output file: {:}.", error);
                    continue;
                }

                println!("Success.");
            }
        }
    }
}
