use clap::Clap;
use std::fs::File;

#[derive(Clap)]
struct Options {
    #[clap(subcommand)]
    subcommand: Subcommand,
    file: String
}

#[derive(Clap)]
enum Subcommand {
    Extract, List(List), Count(Count)
}

#[derive(Clap)]
enum List {
    Files, Directories
}

#[derive(Clap)]
enum Count {
    Files, Directories
}

fn main() {
    let options = Options::parse();
    let file = File::open(options.file).unwrap();

    let dirs_count = libformats::dat::count_dirs(&file).unwrap();
    let dirs = libformats::dat::list_dirs(&file, &dirs_count).unwrap();
    let files = libformats::dat::list_files(&file, &dirs).unwrap();
    let paths = &files.iter().map(|f| f.path.to_owned()).collect::<Vec<String>>();

    match options.subcommand {
        Subcommand::Extract => { unimplemented!(); },
        Subcommand::List(subject) => {
            match subject {
                List::Directories => { println!("{:?}", &dirs.names); },
                List::Files => { 
                    println!("{:?}", paths); 
                }
            }
        },
        Subcommand::Count(subject) => {
            match subject {
                Count::Directories => { println!("{:?}", &dirs_count); },
                Count::Files => { 
                    println!("{:?}", &files.len()); 
                }
            }
        }
    }
}