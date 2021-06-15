use clap::Clap;
use std::fs::File;

#[derive(Clap)]
#[clap()]
struct Options {
    file: String
}

fn main() {
    let options = Options::parse();
    let file = File::open(options.file).unwrap();
    
    let dirs_count = libformats::dat::count_dirs(&file).unwrap();
    println!("Directories: {:?}.", dirs_count);

    let dirs = libformats::dat::list_dirs(&file, dirs_count).unwrap();
    println!("Listing: {:?}.", dirs.names);

    let files = libformats::dat::list_files(&file, dirs).unwrap();
    println!("Files: {:?}.", files.len());

    let paths = files.iter().map(|f| f.path.to_owned()).collect::<Vec<String>>();
    println!("Listing: {:?}.", paths);
}