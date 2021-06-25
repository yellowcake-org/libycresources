use clap::Clap;
use std::fs::File;
use std::io::Write;

#[derive(Clap)]
struct Options {
    #[clap(subcommand)]
    subcommand: Subcommand,
    file: String
}

#[derive(Clap)]
enum Subcommand {
    Extract(Extract), List(List), Count(Count)
}

#[derive(Clap)]
enum List {
    Files, Directories
}

#[derive(Clap)]
enum Count {
    Files, Directories
}

#[derive(Clap)]
struct Extract {
    output: String
}

fn main() {
    let options = Options::parse();
    let file = File::open(options.file).unwrap();

    let dirs_count = libformats::dat::count_dirs(&file).unwrap();
    let dirs = libformats::dat::list_dirs(&file, &dirs_count).unwrap();
    let files = libformats::dat::list_files(&file, &dirs).unwrap();

    match options.subcommand {
        Subcommand::Extract(cmd) => {
            println!("Extracting {:?} files...", &files.len());

            for header in files {
                println!("Extracting {:?}...", &header.path);

                let mut extracted = match libformats::dat::extract(&file, &header) {
                    Ok(value) => value,
                    Err(error) => { println!("Erred: {:?}.", error); continue; }
                };

                let filename = cmd.output.to_owned() + &header.path;
                let path = std::path::Path::new(&filename);
                
                let directory = match path.parent() {
                    None => { println!("Couldn't unwrap path."); continue; }
                    Some(directory) => directory
                };

                if let Err(error) = std::fs::create_dir_all(&directory) { println!("Erred: {:?}.", error); continue; }

                let mut created = match std::fs::File::create(&path) {
                    Err(error) => { println!("Erred: {:?}.", error); continue; },
                    Ok(created) => created
                };

                let written = match created.write(&mut extracted) {
                    Err(error) => { println!("Erred: {:?}.", error); continue; },
                    Ok(value) => value
                } as u32;

                if extracted.len() != written as usize { println!("Written bytes aren't equal to extracted.") }
            }
        },
        Subcommand::List(subject) => {
            match subject {
                List::Directories => { println!("{:?}", &dirs.names) },
                List::Files => {
                    let paths = &files.iter().map(|f| f.path.to_owned()).collect::<Vec<String>>();
                    println!("{:?}", paths)
                }
            }
        },
        Subcommand::Count(subject) => {
            match subject {
                Count::Directories => { println!("{:?}", &dirs_count) },
                Count::Files => { 
                    println!("{:?}", &files.len())
                }
            }
        }
    }
}