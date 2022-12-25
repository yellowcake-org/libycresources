use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors::Error;

use super::File;

pub fn file<S: Read + Seek, O: Write>(source: &mut S, file: &File, output: &mut O) -> Result<(), Error> {
    let plain = file.size as usize;
    let archived = file.range.len();

    source.seek(SeekFrom::Start(file.range.start as u64))?;

    if plain != archived {
        let mut written: usize = 0;
        let mut processed: usize = 0;

        while processed < archived as usize {
            let count = source.read_i16::<BigEndian>()?;
            processed += 2;

            if count == 0 { break; }

            if count < 0 {
                let end = processed + count.abs() as usize;

                while processed < end && written < plain as usize {
                    let byte = source.read_u8()?;

                    processed += 1;
                    written += output.write(&[byte])?;
                }
            } else {
                const MATCH_MIN: u16 = 3;
                const MATCH_MAX: u16 = 18;

                let mut buffer = vec![0x20; 4096];
                let mut offset_r: u16 = buffer.len() as u16 - MATCH_MAX;

                let end = processed + count as usize;
                while processed < end {
                    let mut flags = source.read_u8()? as u16;
                    processed += 1;

                    for _ in 0..8 {
                        if processed >= end { break; }

                        let byte = source.read_u8()?;
                        if (flags & 1) != 0 {
                            processed += 1;
                            written += output.write(&[byte])?;

                            buffer[offset_r as usize] = byte;
                            offset_r += 1;

                            if offset_r >= buffer.len() as u16 { offset_r = 0 }
                        } else {
                            let mut offset_w = byte as u16;
                            processed += 1;

                            let mut length = source.read_u8()? as u16;
                            processed += 1;

                            offset_w |= (0xF0 & length) << 4;
                            length &= 0x0F;

                            for _ in 0..(length + MATCH_MIN) {
                                let byte = buffer[offset_w as usize];

                                buffer[offset_r as usize] = byte;
                                written += output.write(&[byte])?;

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

        if plain != written { return Err(Error::Format); }

        Ok(())
    } else {
        let mut bytes = vec![0u8; archived as usize];

        source.read_exact(&mut bytes)?;
        output.write(&bytes)?;

        Ok(())
    }
}
