use clap::Clap;
use libycresources::dat;

use std::fs::File;
use std::io::Write;

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
    let file = match File::open(options.input) {
        Err(error) => {
            eprintln!("File opening error: {:?}.", error);
            return;
        }
        Ok(value) => value,
    };

    let entries = match dat::list::entries(&file) {
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
    } else if let Some(output) = options.extract {
        for entry in &entries {
            println!("Extracting {:?}...", &entry.path);

            let mut extracted = match dat::extract::entry(&file, &entry) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Extraction error: {:?}.", error);
                    continue;
                }
            };

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
                    eprintln!("File creation error: {:?}.", error);
                    continue;
                }
                Ok(created) => created,
            };

            let written = match created.write(&mut extracted) {
                Err(error) => {
                    eprintln!("File writing error: {:?}.", error);
                    continue;
                }
                Ok(value) => value,
            } as u32;

            if extracted.len() != written as usize {
                eprintln!("Attention! Written bytes aren't equal to extracted.");
                eprintln!("Hence the file is corrupted.")
            }
        }
    } else {
        eprintln!("No command passed.");
    }
}
