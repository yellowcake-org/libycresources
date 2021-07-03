use super::Entry;

use std::convert::TryInto;
use std::mem::size_of;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error<R> {
    Read(R),
    Format,
    Reader,
}

pub fn entries<R, E>(reader: R) -> Result<Vec<Entry>, Error<E>>
where
    R: Fn(std::ops::Range<usize>) -> Result<Vec<u8>, E>,
{
    let mut offset = 0;

    let count = u32::from_be_bytes(
        match reader(offset..offset + size_of::<u32>()).map(|vec| vec.try_into()) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => match value {
                Err(_) => return Err(Error::Reader),
                Ok(value) => value,
            },
        },
    );

    offset += size_of::<u32>();
    offset += 3 * size_of::<u32>(); // skip attributes

    let mut dirs = Vec::new();

    for _ in 0..count {
        let length = u8::from_be_bytes(
            match reader(offset..offset + size_of::<u8>()).map(|vec| vec.try_into()) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => match value {
                    Err(_) => return Err(Error::Reader),
                    Ok(value) => value,
                },
            },
        ) as usize;

        offset += size_of::<u8>();

        let mut name: String = match String::from_utf8(match reader(offset..offset + length) {
            Err(_) => return Err(Error::Reader),
            Ok(value) => value,
        }) {
            Err(_) => return Err(Error::Format),
            Ok(value) => value,
        }
        .split('\\')
        .map(|dir| dir.to_owned() + "/")
        .collect();

        offset += length;

        name.pop();
        dirs.push(name);
    }

    let mut entries = Vec::new();

    for dir in &dirs {
        let file_count = u32::from_be_bytes(
            match reader(offset..offset + size_of::<u32>()).map(|vec| vec.try_into()) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => match value {
                    Err(_) => return Err(Error::Reader),
                    Ok(value) => value,
                },
            },
        );

        offset += size_of::<u32>();
        offset += 3 * size_of::<u32>(); // skip attributes

        for _ in 0..file_count {
            let length = u8::from_be_bytes(
                match reader(offset..offset + size_of::<u8>()).map(|vec| vec.try_into()) {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;

            offset += size_of::<u8>();

            let name: String = match String::from_utf8(match reader(offset..offset + length) {
                Err(_) => return Err(Error::Reader),
                Ok(value) => value,
            }) {
                Err(_) => return Err(Error::Format),
                Ok(value) => value,
            };

            offset += length;
            offset += size_of::<u32>(); // skip attributes

            let start = u32::from_be_bytes(
                match reader(offset..offset + size_of::<u32>()).map(|vec| vec.try_into()) {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            let size = u32::from_be_bytes(
                match reader(offset..offset + size_of::<u32>()).map(|vec| vec.try_into()) {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            let packed_size = u32::from_be_bytes(
                match reader(offset..offset + size_of::<u32>()).map(|vec| vec.try_into()) {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            entries.push(Entry {
                path: String::from_str(&dir).unwrap() + "/" + &name,
                range: start..start + {
                    if packed_size > 0 {
                        packed_size
                    } else {
                        size
                    }
                },
                size: size as usize,
            })
        }
    }

    Ok(entries)
}
