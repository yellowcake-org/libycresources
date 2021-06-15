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

pub fn list_dirs(mut file: &File, count: u32) -> Result<headers::dir::Dir, Error> {
	assert!(count != 0);
	let mut offset: u32 = 4 * 4;

	if let Err(error) = file.seek(std::io::SeekFrom::Start(offset as u64)) {
		return Err(Error::File(error))
	} else {
		let mut names = Vec::new();

		for _ in 0..count {
			const NAME_LENGTH_COUNT: usize = 1;
			let mut name_length_slice: [u8; NAME_LENGTH_COUNT] = [0; NAME_LENGTH_COUNT];
			
			offset += match file.read_exact(&mut name_length_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => NAME_LENGTH_COUNT as u32
			};

			let name_length = u8::from_be_bytes(name_length_slice);
			let mut name_slice = vec![0u8; name_length as usize];
			
			offset += match file.read_exact(&mut name_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => name_length as u32
			};

			names.push(match String::from_utf8(name_slice) {
				Err(error) => return Err(Error::Encoding(error)),
				Ok(name) => name.split("\\").map(|dir| dir.to_owned() + "/").collect()
			});
		}

		return Ok(headers::dir::Dir{ names: names, offset: offset })
	}
}

pub fn list_files(mut file: &File, within: headers::dir::Dir) -> Result<Vec<headers::file::File>, Error> {
	let count = within.names.len();
	assert!(count != 0);

	if let Err(error) = file.seek(std::io::SeekFrom::Start(within.offset as u64)) {
		return Err(Error::File(error))
	} else {
		let mut files = Vec::new();
		
		for dir in within.names {
			let file_count = match fetch_u32(file, None) {
				Err(error) => return Err(error),
				Ok(value) => value
			};

			if let Err(error) = file.seek(std::io::SeekFrom::Current(3 * 4 as i64)) {
				return Err(Error::File(error))
			} else {
				for _ in 0..file_count {
					const NAME_LENGTH_COUNT: usize = 1;
					let mut name_length_slice: [u8; NAME_LENGTH_COUNT] = [0; NAME_LENGTH_COUNT];
					
					match file.read_exact(&mut name_length_slice) {
						Err(error) => return Err(Error::File(error)),
						Ok(_) => NAME_LENGTH_COUNT
					};

					let name_length = u8::from_be_bytes(name_length_slice);
					let mut name_slice = vec![0u8; name_length as usize];
					
					match file.read_exact(&mut name_slice) {
						Err(error) => return Err(Error::File(error)),
						Ok(_) => name_length as usize
					};

					let name = match String::from_utf8(name_slice) {
						Err(error) => return Err(Error::Encoding(error)),
						Ok(name) => name
					};

					let path = String::from_str(&dir).unwrap() + &name;
					const ATTRIBUTES_LENGTH: i64 = 4;

					if let Err(error) = file.seek(std::io::SeekFrom::Current(ATTRIBUTES_LENGTH)) {
						return Err(Error::File(error))
					} else {
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

						files.push(headers::file::File {
							name: name,
							path: path,
							offset: offset,
							size: complex_size
						})
					}
				}
			}
		}

		return Ok(files)		
	}	
}

// MARK: - Private

fn fetch_u32(mut file: &File, offset: Option<u64>) -> Result<u32, Error> {
	if let Some(offset) = offset {
		if let Err(error) = file.seek(std::io::SeekFrom::Start(offset)) {
			return Err(Error::File(error))
		}
	}

	const BYTES_COUNT: usize = 4;
	let mut slice: [u8; BYTES_COUNT] = [0; BYTES_COUNT];
	
	return match file.read_exact(&mut slice) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => Ok(u32::from_be_bytes(slice))
	}
}