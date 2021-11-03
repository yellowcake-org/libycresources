use super::Palette;
use crate::common::graphics::{ColorPixel, Pixel};

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Source,
}

pub fn palette<S: Read + Seek>(source: &mut S) -> Result<Palette, Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(Error::Read(error));
    }

    let scale = 0..64;
    let mut colors: [(usize, usize, usize, bool); 256] = [(0, 0, 0, false); 256];

    for color in &mut colors {
        let mut red_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut red_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let red = u8::from_le_bytes(match red_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut green_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut green_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let green = u8::from_le_bytes(match green_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut blue_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut blue_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let blue = u8::from_le_bytes(match blue_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        if scale.contains(&red) && scale.contains(&green) && scale.contains(&blue) {
            *color = (red, green, blue, true)
        }
    }

    let colors = colors.map(|(red, green, blue, is_mapped)| {
        if is_mapped {
            Some(ColorPixel {
                red: Pixel {
                    value: red,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: green,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: blue,
                    scale: scale.start..scale.end,
                },
            })
        } else {
            None
        }
    });

    Ok(Palette { colors })
}
