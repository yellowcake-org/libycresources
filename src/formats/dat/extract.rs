use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;

use super::File;

#[derive(Debug)]
pub enum Error {
    Write(std::io::Error),
    Source,
    Read(std::io::Error),
    Decompress,
}

pub fn file<S: Read + Seek, O: Write>(
    source: &mut S,
    file: &File,
    output: &mut O,
) -> Result<(), Error> {
    let plain = file.size;
    let archived = file.range.end - file.range.start;

    if let Err(error) = source.seek(SeekFrom::Start(file.range.start as u64)) {
        return Err(Error::Read(error));
    }

    if plain != archived {
        let mut written: usize = 0;
        let mut processed: usize = 0;

        while processed < archived as usize {
            let mut count_bytes = vec![0u8; size_of::<i16>()];
            match source.read_exact(&mut count_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let count = i16::from_be_bytes(match count_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            });
            processed += 2;

            if count == 0 {
                break;
            }

            if count < 0 {
                let end = processed + count.abs() as usize;

                while processed < end && written < plain as usize {
                    let mut byte_bytes = vec![0u8; size_of::<u8>()];
                    match source.read_exact(&mut byte_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let byte = u8::from_be_bytes(match byte_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    });
                    processed += 1;

                    written += match output.write(&[byte]) {
                        Err(error) => return Err(Error::Write(error)),
                        Ok(value) => value,
                    };
                }
            } else {
                const MATCH_MIN: u16 = 3;
                const MATCH_MAX: u16 = 18;

                let mut buffer = vec![0x20; 4096];
                let mut offset_r: u16 = buffer.len() as u16 - MATCH_MAX;

                let end = processed + count as usize;
                while processed < end {
                    let mut flags_bytes = vec![0u8; size_of::<u8>()];
                    match source.read_exact(&mut flags_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let mut flags = u8::from_be_bytes(match flags_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    }) as u16;
                    processed += 1;

                    for _ in 0..8 {
                        if processed >= end {
                            break;
                        }

                        let mut byte_bytes = vec![0u8; size_of::<u8>()];
                        match source.read_exact(&mut byte_bytes) {
                            Err(error) => return Err(Error::Read(error)),
                            Ok(value) => value,
                        };

                        if (flags & 1) != 0 {
                            let byte = u8::from_be_bytes(match byte_bytes.try_into() {
                                Err(_) => return Err(Error::Source),
                                Ok(value) => value,
                            });
                            processed += 1;

                            written += match output.write(&[byte]) {
                                Err(error) => return Err(Error::Write(error)),
                                Ok(value) => value,
                            };

                            buffer[offset_r as usize] = byte;
                            offset_r += 1;

                            if offset_r >= buffer.len() as u16 {
                                offset_r = 0
                            }
                        } else {
                            let mut offset_w = u8::from_be_bytes(match byte_bytes.try_into() {
                                Err(_) => return Err(Error::Source),
                                Ok(value) => value,
                            }) as u16;
                            processed += 1;

                            let mut length_bytes = vec![0u8; size_of::<u8>()];
                            match source.read_exact(&mut length_bytes) {
                                Err(error) => return Err(Error::Read(error)),
                                Ok(value) => value,
                            };

                            let mut length = u8::from_be_bytes(match length_bytes.try_into() {
                                Err(_) => return Err(Error::Source),
                                Ok(value) => value,
                            }) as u16;
                            processed += 1;

                            offset_w |= (0xF0 & length) << 4;
                            length &= 0x0F;

                            for _ in 0..(length + MATCH_MIN) {
                                let byte = buffer[offset_w as usize];

                                buffer[offset_r as usize] = byte;
                                written += match output.write(&[byte]) {
                                    Err(error) => return Err(Error::Write(error)),
                                    Ok(value) => value,
                                };

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

        if plain != written as u32 {
            return Err(Error::Decompress);
        }

        Ok(())
    } else {
        let mut bytes = vec![0u8; archived as usize];
        match source.read_exact(&mut bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        if let Err(error) = output.write(&bytes) {
            return Err(Error::Write(error));
        }

        Ok(())
    }
}
