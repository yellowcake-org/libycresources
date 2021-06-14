pub mod header;

use std::fs::File;
use std::io::{Read, Seek};
use std::string::String;

#[derive(Debug)]
pub enum Error {
	File(std::io::Error), 
	Encoding(std::string::FromUtf8Error)
}

pub fn count_dirs(mut file: &File) -> Result<usize, Error> {
	return match file.seek(std::io::SeekFrom::Start(0)) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => {
			const NUMBER_COUNT: usize = 4;
			let mut slice: [u8; NUMBER_COUNT] = [0; NUMBER_COUNT];
			
			match file.read_exact(&mut slice) {
				Err(error) => Err(Error::File(error)),
				Ok(_) => Ok(u32::from_be_bytes(slice) as usize)
			}
		}
	}
}

pub fn list_dirs(mut file: &File, count: usize) -> Result<header::Dirs, Error> {
	assert!(count != 0);
	let mut offset: u64 = 4 * 4;

	if let Err(error) = file.seek(std::io::SeekFrom::Start(offset)) {
		return Err(Error::File(error))
	} else {
		let mut names = Vec::new();

		for _ in 0..count {
			const NAME_LENGTH_COUNT: usize = 1;
			let mut name_length_slice: [u8; NAME_LENGTH_COUNT] = [0; NAME_LENGTH_COUNT];
			
			offset += match file.read_exact(&mut name_length_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => NAME_LENGTH_COUNT as u64
			};

			let name_length = u8::from_be_bytes(name_length_slice);
			let mut name_slice = vec![0u8; name_length as usize];
			
			offset += match file.read_exact(&mut name_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => name_length as u64
			};

			names.push(match String::from_utf8(name_slice) {
				Err(error) => return Err(Error::Encoding(error)),
				Ok(name) => name
			});
		}

		return Ok(header::Dirs{ names: names, offset: offset })
	}
}