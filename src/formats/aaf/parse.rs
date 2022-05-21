use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors::Error;
use crate::common::types::geometry::Scaled;

use super::{Font, Glyph, Spacing};

pub fn font<S: Read + Seek>(source: &mut S) -> Result<Font, Error> {
    source.seek(SeekFrom::Start(0))?;
    if source.read_i32::<BigEndian>()? != 0x41414646 { return Err(Error::Format); }

    let height = source.read_u16::<BigEndian>()?;
    let h_spacing = source.read_u16::<BigEndian>()?;
    let space_width = source.read_u16::<BigEndian>()?;
    let v_spacing = source.read_u16::<BigEndian>()?;

    let mut sizes = [(0, 0); 256];
    for size in &mut sizes {
        size.0 = source.read_u16::<BigEndian>()?;
        size.1 = source.read_u16::<BigEndian>()?;

        source.seek(SeekFrom::Current(4))?;
    }

    sizes[b' ' as usize].0 = space_width;

    let mut data = sizes
        .map(|(width, height)| (width, height, vec![0u8; width as usize * height as usize]));

    for bytes in &mut data {
        source.read_exact(&mut bytes.2)?;
    }

    let glyphs = data.map(|(width, height, bytes)| Glyph {
        width,
        height,
        dots: bytes
            .iter()
            .map(|byte| Scaled {
                value: *byte as usize,
                scale: usize::MIN..10,
            })
            .collect(),
    });

    return Ok(Font {
        height,
        spacing: Spacing {
            vertical: v_spacing,
            horizontal: h_spacing,
        },
        glyphs,
    });
}
