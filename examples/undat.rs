use clap::Clap;
use libycresources::dat;

use std::fs::File;
use std::io::{Read, Seek, Write};

#[derive(Clap)]
#[clap(name = "undat", version)]
struct Options {
    input: String,
    #[clap(short, long)]
    list: bool,
    #[clap(short, long)]
    extract: Option<String>,
}

fn main() {
    let options = Options::parse();

    let mut file = match File::open(&options.input) {
        Err(error) => {
            eprintln!("Couldn't open input file: {:?}.", error);
            return;
        }
        Ok(value) => value,
    };

    let mut buffer: Vec<u8> = Vec::new();
    if let Err(error) = file.read_to_end(&mut buffer) {
        eprintln!("Couldn't read from input file: {:?}.", error)
    }

    let mut reader = |range: std::ops::Range<usize>| {
        let result: Result<Vec<u8>, ()> = Ok(buffer[range].to_vec());
        result
    };

    let entries = match dat::list::entries(&mut reader) {
        Err(error) => {
            eprintln!("Files listing error: {:?}.", error);
            return;
        }
        Ok(value) => value,
    };

    if options.list {
        for entry in &entries {
            println!("{:}", &entry.path);
        }
    } else if let Some(output) = &options.extract {
        for entry in &entries {
            println!("Extracting {:?}...", &entry.path);

            let root = std::path::Path::new(&output);
            let joined = root.join(&entry.path);
            let path = joined.as_path();

            let directory = match path.parent() {
                None => {
                    eprintln!("Parent for path \"{:?}\" is not valid!", path);
                    continue;
                }
                Some(directory) => directory,
            };

            if let Err(error) = std::fs::create_dir_all(&directory) {
                eprintln!("Directory creation error: {:?}.", error);
                continue;
            }

            let mut created = match std::fs::File::create(&path) {
                Err(error) => {
                    eprintln!("Couldn't create output file: {:?}.", error);
                    continue;
                }
                Ok(created) => created,
            };

            let mut buffer: Vec<u8> = Vec::new();
            if let Err(error) = dat::extract::entry(&mut reader, &entry, &mut |bytes: &[u8]| {
                buffer.extend_from_slice(bytes);
                let result: Result<usize, ()> = Ok(bytes.len());
                result
            }) {
                eprintln!("Extraction error: {:?}.", error);
                continue;
            }

            if let Err(error) = created.write(buffer.as_slice()) {
                eprintln!("Writing file error: {:?}.", error);
                continue;
            }
        }
    } else {
        eprintln!("No command passed.");
    }
}
