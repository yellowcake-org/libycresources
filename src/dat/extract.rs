use super::Entry;

use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
pub enum Error<R, W> {
    Write(W),
    Reader,
    Read(R),
    Decompress,
}

pub fn entry<R, W, RE, WE>(
    reader: &mut R,
    entry: &Entry,
    writer: &mut W,
) -> Result<(), Error<RE, WE>>
where
    R: FnMut(std::ops::Range<usize>) -> Result<Vec<u8>, RE>,
    W: FnMut(&[u8]) -> Result<usize, WE>,
{
    let plain = entry.size;
    let archived = entry.range.end - entry.range.start;

    if plain != archived {
        let mut written: usize = 0;
        let mut processed: usize = 0;

        while processed < archived as usize {
            let count = i16::from_be_bytes(
                match reader(
                    (entry.range.start + processed)
                        ..size_of::<i16>() + (entry.range.start + processed),
                )
                .map(|vec| vec.try_into())
                {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            );
            processed += 2;

            if count == 0 {
                break;
            }

            if count < 0 {
                let end = processed + count.abs() as usize;

                while processed < end && written < plain as usize {
                    let byte = u8::from_be_bytes(
                        match reader(
                            (entry.range.start + processed)
                                ..size_of::<u8>() + (entry.range.start + processed),
                        )
                        .map(|vec| vec.try_into())
                        {
                            Err(error) => return Err(Error::Read(error)),
                            Ok(value) => match value {
                                Err(_) => return Err(Error::Reader),
                                Ok(value) => value,
                            },
                        },
                    );
                    processed += 1;

                    written += match writer(&[byte]) {
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
                    let mut flags: u16 = u8::from_be_bytes(
                        match reader(
                            (entry.range.start + processed)
                                ..size_of::<u8>() + (entry.range.start + processed),
                        )
                        .map(|vec| vec.try_into())
                        {
                            Err(error) => return Err(Error::Read(error)),
                            Ok(value) => match value {
                                Err(_) => return Err(Error::Reader),
                                Ok(value) => value,
                            },
                        },
                    ) as u16;
                    processed += 1;

                    for _ in 0..8 {
                        if processed >= end {
                            break;
                        }

                        if (flags & 1) != 0 {
                            let byte = u8::from_be_bytes(
                                match reader(
                                    (entry.range.start + processed)
                                        ..size_of::<u8>() + (entry.range.start + processed),
                                )
                                .map(|vec| vec.try_into())
                                {
                                    Err(error) => return Err(Error::Read(error)),
                                    Ok(value) => match value {
                                        Err(_) => return Err(Error::Reader),
                                        Ok(value) => value,
                                    },
                                },
                            );
                            processed += 1;

                            written += match writer(&[byte]) {
                                Err(error) => return Err(Error::Write(error)),
                                Ok(value) => value,
                            };

                            buffer[offset_r as usize] = byte;
                            offset_r += 1;

                            if offset_r >= buffer.len() as u16 {
                                offset_r = 0
                            }
                        } else {
                            let mut offset_w: u16 = u8::from_be_bytes(
                                match reader(
                                    (entry.range.start + processed)
                                        ..size_of::<u8>() + (entry.range.start + processed),
                                )
                                .map(|vec| vec.try_into())
                                {
                                    Err(error) => return Err(Error::Read(error)),
                                    Ok(value) => match value {
                                        Err(_) => return Err(Error::Reader),
                                        Ok(value) => value,
                                    },
                                },
                            ) as u16;
                            processed += 1;

                            let mut length: u16 = u8::from_be_bytes(
                                match reader(
                                    (entry.range.start + processed)
                                        ..size_of::<u8>() + (entry.range.start + processed),
                                )
                                .map(|vec| vec.try_into())
                                {
                                    Err(error) => return Err(Error::Read(error)),
                                    Ok(value) => match value {
                                        Err(_) => return Err(Error::Reader),
                                        Ok(value) => value,
                                    },
                                },
                            ) as u16;
                            processed += 1;

                            offset_w |= (0xF0 & length) << 4;
                            length &= 0x0F;

                            for _ in 0..(length + MATCH_MIN) {
                                let byte = buffer[offset_w as usize];

                                buffer[offset_r as usize] = byte;
                                written += match writer(&[byte]) {
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

        if plain != written {
            return Err(Error::Decompress);
        }

        Ok(())
    } else {
        let bytes = match reader(entry.range.start..entry.range.end) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        if let Err(error) = writer(&bytes) {
            return Err(Error::Write(error));
        }

        Ok(())
    }
}
