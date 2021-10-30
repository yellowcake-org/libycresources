use super::{Font, Glyph, Pixel, Spacing};

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Format,
    Source,
}

pub fn font<S: Read + Seek>(source: &mut S) -> Result<Font, Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(Error::Read(error));
    }

    let mut signature_bytes = vec![0u8; 4];
    match source.read_exact(&mut signature_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    if signature_bytes == b"AAFF" {
        let mut height_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut height_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let height = u16::from_be_bytes(match height_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut h_spacing_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut h_spacing_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let h_spacing = u16::from_be_bytes(match h_spacing_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut space_width_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut space_width_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let space_width = u16::from_be_bytes(match space_width_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut v_spacing_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut v_spacing_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let v_spacing = u16::from_be_bytes(match v_spacing_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut sizes = [(0, 0); 256];
        for &(mut size) in sizes.iter() {
            let mut width_bytes = vec![0u8; size_of::<u16>()];
            match source.read_exact(&mut width_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            size.0 = u16::from_be_bytes(match width_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

            let mut height_bytes = vec![0u8; size_of::<u16>()];
            match source.read_exact(&mut height_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            size.1 = u16::from_be_bytes(match height_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

            let mut _offset_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut _offset_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };
        }

        sizes[b' ' as usize].0 = space_width;

        let mut data = sizes.map(|(width, height)| (width, height, vec![0u8; width * height]));
        for bytes in &mut data {
            match source.read_exact(&mut bytes.2) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };
        }

        let glyphs = data.map(|(width, height, bytes)| Glyph {
            width,
            height,
            pixels: bytes
                .iter()
                .map(|byte| Pixel {
                    value: *byte as usize,
                    scale: 9,
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

    Err(Error::Format)
}
