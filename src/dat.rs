pub mod headers;

use std::fs::File;
use std::io::{Read, Seek, Write};

use std::string::String;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
	File(std::io::Error),
	Corrupted,
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

pub fn extract(mut file: &File, header: &headers::file::File, output: &String) -> Result<(), Error> {
	let filename = output.to_owned() + &header.path;
	let path = std::path::Path::new(&filename);
	
	let directory = match path.parent() {
		None => return Err(Error::Corrupted),
		Some(directory) => directory
	};

	if let Err(error) = std::fs::create_dir_all(&directory) { return Err(Error::File(error)) }

	let mut created = match std::fs::File::create(&path) {
		Err(error) => return Err(Error::File(error)),
		Ok(created) => created
	};

	if let Err(error) = file.seek(std::io::SeekFrom::Start(header.offset as u64)) { return Err(Error::File(error)) }
	
	match header.size {
		headers::file::Size::Plain(length) => {
			let mut bytes = vec![0u8; length as usize];
			if let Err(error) = file.read_exact(&mut bytes) { return Err(Error::File(error)) }

			let written = match created.write(&bytes) {
				Err(error) => return Err(Error::File(error)),
				Ok(size) => size as u32
			};

			if length != written { return Err(Error::Corrupted) }

			return Ok(())
		},
		headers::file::Size::Packed { compressed, plain } => {
			let mut bytes = vec![0u8; compressed as usize];
			let mut decompressed = vec![0u8; plain as usize];
			
			if let Err(error) = file.read_exact(&mut bytes) { return Err(Error::File(error)) }

			const DICT_SIZE: u16 = 4096;

			const MATCH_MIN: u16 = 3;
			const MATCH_MAX: u16 = 18;

			let mut offset_r: u16;
			let mut offset_w: u16;

			let mut count: i16;
			let mut flags: u16;
			let mut length: u16;

			let mut idx: usize = 0;
			let mut ddx: usize = 0;

			while idx < compressed as usize {
				count = i16::from_be_bytes([bytes[idx], bytes[idx + 1]]);
				idx += 2;

				if count == 0 { break }
				let end = idx + count.abs() as usize;

				if count < 0 {
					while idx < end && ddx < plain as usize {
						let byte = bytes[idx];
						idx += 1;

						decompressed[ddx] = byte;
						ddx += 1;
					}
				} else {
					offset_r = DICT_SIZE - MATCH_MAX;
					let mut buffer = vec![0x20; DICT_SIZE as usize];

					while idx < end {
						flags = bytes[idx] as u16;
						idx += 1;

						for _ in 0..8 {
							if idx >= end { break }
							
							if (flags & 1) != 0 {
								let byte = bytes[idx];
								idx += 1;

								decompressed[ddx] = byte;
								ddx += 1;

								buffer[offset_r as usize] = byte;
								offset_r += 1;

								if offset_r >= DICT_SIZE { offset_r = 0 }
							} else {
								offset_w = bytes[idx] as u16;
								idx += 1;

								length = bytes[idx] as u16;
								idx += 1;

								offset_w = offset_w | ((0xF0 & length) << 4);
                            	length &= 0x0F;

                            	for _ in 0..(length + MATCH_MIN) {
                            		let byte = buffer[offset_w as usize];
                            		
                            		decompressed[ddx] = byte;
                            		ddx += 1;

                            		buffer[offset_r as usize] = byte;

                            		offset_w += 1;
                            		offset_r += 1;

                            		if offset_r >= DICT_SIZE { offset_r = 0 }
	                                if offset_w >= DICT_SIZE { offset_w = 0 }
                            	}
							}

							flags >>= 1;
						}
					}
				}
			}

			let written = match created.write(&decompressed) {
				Err(error) => return Err(Error::File(error)),
				Ok(size) => size as u32
			};

			if plain != written { return Err(Error::Corrupted) }

			return Ok(())
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