pub mod headers;

use std::fs::File;
use std::io::{Read, Seek};

use std::string::String;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
	File(std::io::Error), 
	Encoding(std::string::FromUtf8Error)
}

pub fn count_dirs(file: &File) -> Result<u32, Error> {
	return fetch_u32(file, Some(0));
}

pub fn list_dirs(mut file: &File, count: &u32) -> Result<headers::dir::Dir, Error> {
	assert!(*count != 0);

	if let Err(error) = file.seek(std::io::SeekFrom::Start(4 * 4 as u64)) { return Err(Error::File(error)) }

	let mut names = Vec::new();

	for _ in 0..*count {
		let mut name: String = match fetch_string(file, None) {
			Err(error) => return Err(error),
			Ok(value) => value
		}
		.split("\\")
		.map(|dir| dir.to_owned() + "/")
		.collect();

		name.pop();
		names.push(name);
	}

	let offset = match file.seek(std::io::SeekFrom::Current(0)) {
		Err(error) => return Err(Error::File(error)),
		Ok(value) => value
	};

	return Ok(headers::dir::Dir{ names: names, offset: offset })
}

pub fn list_files(mut file: &File, within: &headers::dir::Dir) -> Result<Vec<headers::file::File>, Error> {
	let count = within.names.len();
	assert!(count != 0);

	if let Err(error) = file.seek(std::io::SeekFrom::Start(within.offset as u64)) { return Err(Error::File(error)) }

	let mut files = Vec::new();
		
	for dir in &within.names {
		let file_count = match fetch_u32(file, None) {
			Err(error) => return Err(error),
			Ok(value) => value
		};

		if let Err(error) = file.seek(std::io::SeekFrom::Current(3 * 4 as i64)) { return Err(Error::File(error)) }

		for _ in 0..file_count {
			let name = match fetch_string(file, None) {
				Err(error) => return Err(error),
				Ok(value) => value
			};

			let path = String::from_str(&dir).unwrap() + "/" + &name;

			if let Err(error) = file.seek(std::io::SeekFrom::Current(4)) { return Err(Error::File(error)) }

			let offset = match fetch_u32(file, None) {
				Err(error) => return Err(error),
				Ok(value) => value
			};

			let size = match fetch_u32(file, None) {
				Err(error) => return Err(error),
				Ok(value) => value
			};

			let packed_size = match fetch_u32(file, None) {
				Err(error) => return Err(error),
				Ok(value) => value
			};

			let complex_size = if packed_size > 0 { 
				headers::file::Size::Packed { compressed: packed_size, plain: size } 
			} else { 
				headers::file::Size::Plain(size) 
			};

			files.push(headers::file::File { name: name, path: path, offset: offset, size: complex_size })
		}
	}

	return Ok(files)
}

// MARK: - Private

fn offset_if_needed(mut file: &File, offset: Option<u64>) -> Result<(), Error> {
	if let Some(offset) = offset {
		if let Err(error) = file.seek(std::io::SeekFrom::Start(offset)) { return Err(Error::File(error)) }
	}

	return Ok(())
}

fn fetch_u32(mut file: &File, offset: Option<u64>) -> Result<u32, Error> {
	if let Err(error) = offset_if_needed(file, offset) { return Err(error) }

	const BYTES_COUNT: usize = 4;
	let mut slice: [u8; BYTES_COUNT] = [0; BYTES_COUNT];
	
	return match file.read_exact(&mut slice) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => Ok(u32::from_be_bytes(slice))
	}
}

fn fetch_string(mut file: &File, offset: Option<u64>) -> Result<String, Error> {
	if let Err(error) = offset_if_needed(file, offset) { return Err(error) }

	const LENGTH_BYTE_COUNT: usize = 1;
	let mut string_length_slice: [u8; LENGTH_BYTE_COUNT] = [0; LENGTH_BYTE_COUNT];
	
	if let Err(error) = file.read_exact(&mut string_length_slice) { return Err(Error::File(error)) }

	let string_length = u8::from_be_bytes(string_length_slice);
	let mut string_slice = vec![0u8; string_length as usize];
	
	if let Err(error) = file.read_exact(&mut string_slice) { return Err(Error::File(error)) }

	return match String::from_utf8(string_slice) {
		Err(error) => return Err(Error::Encoding(error)),
		Ok(string) => Ok(string)
	};
}