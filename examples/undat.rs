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

    let mut reader = |range: std::ops::Range<usize>| {
        if let Err(error) = file.seek(std::io::SeekFrom::Start(range.start as u64)) {
            return Err(error);
        }

        let mut buffer = vec![0u8; range.end - range.start];
        match file.read(&mut buffer) {
            Err(error) => Err(error),
            Ok(_) => Ok(buffer),
        }
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

            let mut writer = |bytes: &[u8]| created.write(bytes);

            if let Err(error) = dat::extract::entry(&mut reader, &entry, &mut writer) {
                eprintln!("Extraction error: {:?}.", error)
            }
        }
    } else {
        eprintln!("No command passed.");
    }
}
