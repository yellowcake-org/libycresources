pub mod headers;

use std::fs::File;
use std::io::{Read, Seek};

use std::string::String;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
	File(std::io::Error),
	Decompression,
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

pub fn extract(mut file: &File, header: &headers::file::File) -> Result<Vec<u8>, Error> {
	if let Err(error) = file.seek(std::io::SeekFrom::Start(header.offset as u64)) { return Err(Error::File(error)) }
	
	match header.size {
		headers::file::Size::Plain(length) => {
			let mut bytes = vec![0u8; length as usize];
			if let Err(error) = file.read_exact(&mut bytes) { return Err(Error::File(error)) }

			return Ok(bytes)
		},
		headers::file::Size::Packed { compressed, plain } => {
			let mut output: Vec<u8> = Vec::new();
			let mut read: usize = 0;

			while read < compressed as usize {
				let count = match fetch_i16(file, None) {
					Err(error) => return Err(error),
					Ok(value) => value
				};
				read += 2;

				if count == 0 { break }

				if count < 0 {
					let end = read + count.abs() as usize;
					
					while read < end && output.len() < plain as usize {
						output.push(match fetch_u8(file, None) {
							Err(error) => return Err(error),
							Ok(value) => value
						});

						read += 1;
					}
				} else {
					const MATCH_MIN: u16 = 3;
					const MATCH_MAX: u16 = 18;

					let mut buffer = vec![0x20; 4096];
					let mut offset_r: u16 = buffer.len() as u16 - MATCH_MAX;

					let end = read + count as usize;
					while read < end {
						let mut flags = match fetch_u8(file, None) {
							Err(error) => return Err(error),
							Ok(value) => value
						} as u16;
						read += 1;

						for _ in 0..8 {
							if read >= end { break }
							
							if (flags & 1) != 0 {
								let byte = match fetch_u8(file, None) {
									Err(error) => return Err(error),
									Ok(value) => value
								};
								read += 1;

								output.push(byte);

								buffer[offset_r as usize] = byte;
								offset_r += 1;

								if offset_r >= buffer.len() as u16 { offset_r = 0 }
							} else {
								let mut offset_w = match fetch_u8(file, None) {
									Err(error) => return Err(error),
									Ok(value) => value
								} as u16;
								read += 1;

								let mut length = match fetch_u8(file, None) {
									Err(error) => return Err(error),
									Ok(value) => value
								} as u16;
								read += 1;

								offset_w = offset_w | ((0xF0 & length) << 4);
                            	length &= 0x0F;

                            	for _ in 0..(length + MATCH_MIN) {
                            		let byte = buffer[offset_w as usize];
                            		
                            		output.push(byte);
                            		buffer[offset_r as usize] = byte;

                            		offset_w += 1;
                            		offset_r += 1;

                            		if offset_r >= buffer.len() as u16 { offset_r = 0 }
	                                if offset_w >= buffer.len() as u16 { offset_w = 0 }
                            	}
							}

							flags >>= 1;
						}
					}
				}
			}

			if plain != output.len() as u32 { return Err(Error::Decompression) }

			return Ok(output)
		}
	}
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

	const COUNT: usize = std::mem::size_of::<u32>();
	let mut slice: [u8; COUNT] = [0; COUNT];
	
	return match file.read_exact(&mut slice) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => Ok(u32::from_be_bytes(slice))
	}
}

fn fetch_i16(mut file: &File, offset: Option<u64>) -> Result<i16, Error> {
	if let Err(error) = offset_if_needed(file, offset) { return Err(error) }

	const COUNT: usize = std::mem::size_of::<i16>();
	let mut slice: [u8; COUNT] = [0; COUNT];

	return match file.read_exact(&mut slice) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => Ok(i16::from_be_bytes(slice))
	}
}

fn fetch_u8(mut file: &File, offset: Option<u64>) -> Result<u8, Error> {
	if let Err(error) = offset_if_needed(file, offset) { return Err(error) }

	const COUNT: usize = std::mem::size_of::<u8>();
	let mut slice: [u8; COUNT] = [0; COUNT];

	return match file.read_exact(&mut slice) {
		Err(error) => Err(Error::File(error)),
		Ok(_) => Ok(u8::from_be_bytes(slice))
	}
}

fn fetch_string(mut file: &File, offset: Option<u64>) -> Result<String, Error> {
	let string_length = match fetch_u8(file, offset) {
		Err(error) => return Err(error),
		Ok(value) => value
	};
	
	let mut string_slice = vec![0u8; string_length as usize];
	if let Err(error) = file.read_exact(&mut string_slice) { return Err(Error::File(error)) }

	return match String::from_utf8(string_slice) {
		Err(error) => return Err(Error::Encoding(error)),
		Ok(string) => Ok(string)
	};
}