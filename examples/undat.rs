use clap::Clap;

use std::fs::File;
use std::io::Write;

#[derive(Clap)]
#[clap(version, author)]
struct Options {
	input: String,
	#[clap(subcommand)]
	subcommand: Subcommand
}

#[derive(Clap)]
enum Subcommand {
	/// Lists all directories contained within provided file.
	Dirs,
	/// Lists all files, including full path. Filtered, if directory specified.
	Files(Files),
	/// Extracts all contents to destination directory.
	Unpack(Unpack)
}

#[derive(Clap)]
struct Unpack {
	/// Destination directory.
	output: String
}

#[derive(Clap)]
struct Files {
	/// If presented, list files within this directory only.
	directory: Option<String>
}

fn main() {
	let options = Options::parse();
	let file = match File::open(options.input) {
		Err(error) => { eprintln!("File opening error: {:?}.", error); return },
		Ok(value) => value
	};

	let dirs = match libformats::dat::dirs(&file) {
		Err(error) => { eprintln!("Dirs listing error: {:?}.", error); return },
		Ok(value) => value
	};
	let files = match libformats::dat::files(&file, &dirs) {
		Err(error) => { eprintln!("Files listing error: {:?}.", error); return },
		Ok(value) => value
	};

	match options.subcommand {
		Subcommand::Dirs => {
			for (idx, dir) in dirs.names.iter().enumerate() {
				println!("[{:?}]: {:?}", idx, dir);
			}
		},
		Subcommand::Files(options) => {
			for (idx, file) in files.iter().enumerate() {
				if let Some(name) = &options.directory {
					if !file.path.starts_with(name) { continue }
				}

				println!("[{:?}]: {:?}", idx, file.path);
			}
		},
		Subcommand::Unpack(cmd) => {
			for header in files {
				println!("Extracting {:?}...", &header.path);

				let mut extracted = match libformats::dat::bytes(&file, &header) {
					Ok(value) => value,
					Err(error) => { eprintln!("Extraction error: {:?}.", error); continue }
				};

				let root = std::path::Path::new(&cmd.output);
				let joined = root.join(header.path);
				let path = joined.as_path();
				
				let directory = match path.parent() {
					None => { eprintln!("Parent for path \"{:?}\" is not valid!", path); continue }
					Some(directory) => directory
				};

				if let Err(error) = std::fs::create_dir_all(&directory) { 
					eprintln!("Directory creation error: {:?}.", error);
					continue
				}

				let mut created = match std::fs::File::create(&path) {
					Err(error) => { eprintln!("File creation error: {:?}.", error); continue },
					Ok(created) => created
				};

				let written = match created.write(&mut extracted) {
					Err(error) => { eprintln!("File writing error: {:?}.", error); continue },
					Ok(value) => value
				} as u32;

				if extracted.len() != written as usize { 
					eprintln!("Attention! Written bytes aren't equal to extracted.");
					eprintln!("Hence the file is corrupted.") 
				}
			}
		}
	}
}