use super::fetch;

use super::Entry;
use super::Error;

use std::fs::File;
use std::io::Read;
use std::io::Seek;

pub fn entry(mut file: &File, entry: &Entry) -> Result<Vec<u8>, Error> {
    if let Err(error) = file.seek(std::io::SeekFrom::Start(entry.range.start as u64)) {
        return Err(Error::Read(error));
    }

    let plain = entry.size;
    let archived = entry.range.end - entry.range.start;

    if plain != archived {
        let mut output: Vec<u8> = Vec::new();
        let mut processed: usize = 0;

        while processed < archived as usize {
            let count = match fetch::i16(file, None) {
                Err(error) => return Err(error),
                Ok(value) => value,
            };
            processed += 2;

            if count == 0 {
                break;
            }

            if count < 0 {
                let end = processed + count.abs() as usize;

                while processed < end && output.len() < plain as usize {
                    output.push(match fetch::u8(file, None) {
                        Err(error) => return Err(error),
                        Ok(value) => value,
                    });

                    processed += 1;
                }
            } else {
                const MATCH_MIN: u16 = 3;
                const MATCH_MAX: u16 = 18;

                let mut buffer = vec![0x20; 4096];
                let mut offset_r: u16 = buffer.len() as u16 - MATCH_MAX;

                let end = processed + count as usize;
                while processed < end {
                    let mut flags = match fetch::u8(file, None) {
                        Err(error) => return Err(error),
                        Ok(value) => value,
                    } as u16;
                    processed += 1;

                    for _ in 0..8 {
                        if processed >= end {
                            break;
                        }

                        if (flags & 1) != 0 {
                            let byte = match fetch::u8(file, None) {
                                Err(error) => return Err(error),
                                Ok(value) => value,
                            };
                            processed += 1;

                            output.push(byte);

                            buffer[offset_r as usize] = byte;
                            offset_r += 1;

                            if offset_r >= buffer.len() as u16 {
                                offset_r = 0
                            }
                        } else {
                            let mut offset_w = match fetch::u8(file, None) {
                                Err(error) => return Err(error),
                                Ok(value) => value,
                            } as u16;
                            processed += 1;

                            let mut length = match fetch::u8(file, None) {
                                Err(error) => return Err(error),
                                Ok(value) => value,
                            } as u16;
                            processed += 1;

                            offset_w |= (0xF0 & length) << 4;
                            length &= 0x0F;

                            for _ in 0..(length + MATCH_MIN) {
                                let byte = buffer[offset_w as usize];

                                output.push(byte);
                                buffer[offset_r as usize] = byte;

                                offset_w += 1;
                                offset_r += 1;

                                if offset_r >= buffer.len() as u16 {
                                    offset_r = 0
                                }
                                if offset_w >= buffer.len() as u16 {
                                    offset_w = 0
                                }
                            }
                        }

                        flags >>= 1;
                    }
                }
            }
        }

        if plain != output.len() {
            return Err(Error::Decompress);
        }

        Ok(output)
    } else {
        let mut bytes = vec![0u8; plain];
        if let Err(error) = file.read_exact(&mut bytes) {
            return Err(Error::Read(error));
        }

        Ok(bytes)
    }
}
