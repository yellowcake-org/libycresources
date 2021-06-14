mod header;
use std::io::Read;

pub fn header(mut file: std::fs::File) -> header::Header {
	let mut directories_count_slice: [u8; 4] = [0; 4];
	let _ = file.read(&mut directories_count_slice);

	return header::Header{ directories: u32::from_be_bytes(directories_count_slice) };
}