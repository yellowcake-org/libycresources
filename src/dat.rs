mod header;

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom::*;

pub fn header(mut file: std::fs::File) -> header::Header {
	let mut directories_count_slice: [u8; 4] = [0; 4];
	let _ = file.read(&mut directories_count_slice);
	let directories_count = u32::from_be_bytes(directories_count_slice);

	let _ = file.seek(Current(3 * 4));
	for _ in 0..directories_count {
		let mut name_length_slice: [u8; 1] = [0; 1];
		let _ = file.read(&mut name_length_slice);
		let name_length = u8::from_be_bytes(name_length_slice);

		let mut name_slice: Vec<u8> = Vec::with_capacity(name_length as usize);
		let _ = file.by_ref().take(name_length as u64).read_to_end(&mut name_slice).unwrap();
		let name = std::str::from_utf8(&name_slice).unwrap();

		println!("{:?}", name);
	}

	return header::Header{ directories_count: directories_count };
}