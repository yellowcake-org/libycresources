use super::super::platform::Reader;
use super::{Directory, File};

use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
pub enum Error<R> {
    Read(R),
    Format,
    Reader,
}

pub fn entries<R, E>(reader: &mut R) -> Result<Option<Directory>, Error<E>>
where
    R: Reader<E>,
{
    let mut offset = 0;

    let count = u32::from_be_bytes(
        match reader
            .read(offset..offset + size_of::<u32>())
            .map(|vec| vec.try_into())
        {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => match value {
                Err(_) => return Err(Error::Reader),
                Ok(value) => value,
            },
        },
    );

    if count == 0 {
        return Ok(None);
    }

    offset += size_of::<u32>();
    offset += 3 * size_of::<u32>(); // skip attributes

    let mut tree_paths = Vec::with_capacity(count as usize);
    let mut tree: Directory = Directory {
        name: String::from("."),
        files: Vec::new(),
        children: Vec::new(),
    };

    for index in 0..count as usize {
        let length = u8::from_be_bytes(
            match reader
                .read(offset..offset + size_of::<u8>())
                .map(|vec| vec.try_into())
            {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => match value {
                    Err(_) => return Err(Error::Reader),
                    Ok(value) => value,
                },
            },
        ) as usize;

        offset += size_of::<u8>();

        let mut path = match String::from_utf8(match reader.read(offset..offset + length) {
            Err(_) => return Err(Error::Reader),
            Ok(value) => value,
        }) {
            Err(_) => return Err(Error::Format),
            Ok(value) => value,
        };

        offset += length;

        // i have no idea why original Fallout™ archives use . for root folder and
        // do not use .\ at start for all it's children, so...
        if path != "." {
            path = String::from(".\\") + &path;
        }

        let mut current: &mut Directory = &mut tree;
        let mut index_path = Vec::new();

        for (level, component) in path.split('\\').enumerate() {
            if level > 0 {
                let found_index = current
                    .children
                    .iter()
                    .enumerate()
                    .find(|n| n.1.name == component)
                    .map(|v| v.0);
                if let Some(existed_index) = found_index {
                    current = &mut current.children[existed_index];
                    index_path.push(Some(existed_index));
                } else {
                    current.children.push(Directory {
                        name: String::from(component),
                        files: Vec::new(),
                        children: Vec::new(),
                    });

                    current = current.children.last_mut().unwrap();
                    index_path.push(Some(current.children.iter().count()));
                }
            } else {
                index_path.push(None);
            }
        }

        tree_paths[index] = index_path;
    }

    for path in &tree_paths {
        let mut directory = &mut tree;
        for component in path {
            if let Some(index) = component {
                directory = &mut directory.children[*index];
            }
        }

        let file_count = u32::from_be_bytes(
            match reader
                .read(offset..offset + size_of::<u32>())
                .map(|vec| vec.try_into())
            {
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
                match reader
                    .read(offset..offset + size_of::<u8>())
                    .map(|vec| vec.try_into())
                {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;

            offset += size_of::<u8>();

            let name: String = match String::from_utf8(match reader.read(offset..offset + length) {
                Err(_) => return Err(Error::Reader),
                Ok(value) => value,
            }) {
                Err(_) => return Err(Error::Format),
                Ok(value) => value,
            };

            offset += length;
            offset += size_of::<u32>(); // skip attributes

            let start = u32::from_be_bytes(
                match reader
                    .read(offset..offset + size_of::<u32>())
                    .map(|vec| vec.try_into())
                {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            let size = u32::from_be_bytes(
                match reader
                    .read(offset..offset + size_of::<u32>())
                    .map(|vec| vec.try_into())
                {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            let packed_size = u32::from_be_bytes(
                match reader
                    .read(offset..offset + size_of::<u32>())
                    .map(|vec| vec.try_into())
                {
                    Err(error) => return Err(Error::Read(error)),
                    Ok(value) => match value {
                        Err(_) => return Err(Error::Reader),
                        Ok(value) => value,
                    },
                },
            ) as usize;
            offset += size_of::<u32>();

            directory.files.push(File {
                name: name,
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

    Ok(Some(tree))
}
