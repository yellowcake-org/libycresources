use clap::Clap;
use std::fs::File;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Alexander O. <me@0xceed.com>")]
struct Options {
    file: String
}

fn main() {
	let options = Options::parse();
    let file = File::open(options.file).unwrap();
    let header = libformats::dat::header(file);

    println!("Header: {:?}", header);
}