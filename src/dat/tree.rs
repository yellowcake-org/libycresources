pub mod iterator;

use super::{Directory, File};

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Format,
    Source,
}

pub fn read<S: Read + Seek>(source: &mut S) -> Result<Option<Directory>, Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(Error::Read(error));
    }

    let mut count_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut count_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let count = u32::from_be_bytes(match count_bytes.try_into() {
        Err(_) => return Err(Error::Source),
        Ok(value) => value,
    });

    if count == 0 {
        return Ok(None);
    }

    // skip attributes
    let mut skip_bytes = vec![0u8; 3 * size_of::<u32>()];
    if let Err(error) = source.read_exact(&mut skip_bytes) {
        return Err(Error::Read(error));
    }

    let mut tree_paths = Vec::with_capacity(count as usize);
    let mut tree: Directory = Directory {
        name: String::from("."),
        files: Vec::new(),
        children: Vec::new(),
    };

    for _ in 0..count as usize {
        let mut length_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut length_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let length = u8::from_be_bytes(match length_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut path_bytes = vec![0u8; length];
        match source.read_exact(&mut path_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let mut path = match String::from_utf8(match path_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) {
            Err(_) => return Err(Error::Format),
            Ok(value) => value,
        };

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
                    index_path.push(Some(existed_index));
                    current = &mut current.children[existed_index];
                } else {
                    current.children.push(Directory {
                        name: String::from(component),
                        files: Vec::new(),
                        children: Vec::new(),
                    });

                    index_path.push(Some(current.children.len() - 1));
                    current = current.children.last_mut().unwrap();
                }
            } else {
                index_path.push(None);
            }
        }

        tree_paths.push(index_path);
    }

    for path in &tree_paths {
        let mut directory = &mut tree;
        for component in path {
            if let Some(index) = component {
                directory = &mut directory.children[*index];
            }
        }

        let mut file_count_bytes = vec![0u8; size_of::<u32>()];
        match source.read_exact(&mut file_count_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let file_count = u32::from_be_bytes(match file_count_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        // skip attributes
        if let Err(error) = source.read_exact(&mut skip_bytes) {
            return Err(Error::Read(error));
        }

        for _ in 0..file_count {
            let mut length_bytes = vec![0u8; size_of::<u8>()];
            match source.read_exact(&mut length_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let length = u8::from_be_bytes(match length_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

            let mut name_bytes = vec![0u8; length];
            match source.read_exact(&mut name_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let name = match String::from_utf8(match name_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) {
                Err(_) => return Err(Error::Format),
                Ok(value) => value,
            };

            // skip attributes
            let mut file_skip_bytes = vec![0u8; size_of::<u32>()];
            if let Err(error) = source.read_exact(&mut file_skip_bytes) {
                return Err(Error::Read(error));
            }

            let mut start_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut start_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let start = u32::from_be_bytes(match start_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

            let mut size_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut size_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let size = u32::from_be_bytes(match size_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

            let mut packed_size_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut packed_size_bytes) {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => value,
            };

            let packed_size = u32::from_be_bytes(match packed_size_bytes.try_into() {
                Err(_) => return Err(Error::Source),
                Ok(value) => value,
            }) as usize;

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
