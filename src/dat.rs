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

pub fn list_dirs(mut file: &File, count: usize) -> Result<headers::dir::Dir, Error> {
	assert!(count != 0);
	let mut offset: usize = 4 * 4;

	if let Err(error) = file.seek(std::io::SeekFrom::Start(offset as u64)) {
		return Err(Error::File(error))
	} else {
		let mut names = Vec::new();

		for _ in 0..count {
			const NAME_LENGTH_COUNT: usize = 1;
			let mut name_length_slice: [u8; NAME_LENGTH_COUNT] = [0; NAME_LENGTH_COUNT];
			
			offset += match file.read_exact(&mut name_length_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => NAME_LENGTH_COUNT
			};

			let name_length = u8::from_be_bytes(name_length_slice);
			let mut name_slice = vec![0u8; name_length as usize];
			
			offset += match file.read_exact(&mut name_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => name_length as usize
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
			const NUMBER_COUNT: usize = 4;
			let mut file_count_slice: [u8; NUMBER_COUNT] = [0; NUMBER_COUNT];
			
			let file_count = match file.read_exact(&mut file_count_slice) {
				Err(error) => return Err(Error::File(error)),
				Ok(_) => u32::from_be_bytes(file_count_slice) as usize
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
						const OFFSET_SLICE_COUNT: usize = 4;
						let mut offset_slice: [u8; OFFSET_SLICE_COUNT] = [0; OFFSET_SLICE_COUNT];
						
						let offset = match file.read_exact(&mut offset_slice) {
							Err(error) => return Err(Error::File(error)),
							Ok(_) => u32::from_be_bytes(offset_slice) as usize
						};

						const SIZE_SLICE_COUNT: usize = 4;
						let mut size_slice: [u8; SIZE_SLICE_COUNT] = [0; SIZE_SLICE_COUNT];
						
						let size = match file.read_exact(&mut size_slice) {
							Err(error) => return Err(Error::File(error)),
							Ok(_) => u32::from_be_bytes(size_slice) as usize
						};

						const PACKED_SIZE_SLICE_COUNT: usize = 4;
						let mut packed_size_slice: [u8; PACKED_SIZE_SLICE_COUNT] = [0; PACKED_SIZE_SLICE_COUNT];
						
						let packed_size = match file.read_exact(&mut packed_size_slice) {
							Err(error) => return Err(Error::File(error)),
							Ok(_) => u32::from_be_bytes(packed_size_slice) as usize
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