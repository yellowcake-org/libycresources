use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors::Error;

use super::{Directory, File};

pub mod iterator;

pub fn tree<S: Read + Seek>(source: &mut S) -> Result<Option<Directory>, Error> {
    source.seek(SeekFrom::Start(0))?;

    let count = source.read_u32::<BigEndian>()? as usize;
    if count == 0 { return Ok(None); }

    source.seek(SeekFrom::Current(3 * 4))?;

    let mut tree_paths = Vec::with_capacity(count);
    let mut tree: Directory = Directory {
        name: String::from("."),
        files: Vec::new(),
        children: Vec::new(),
    };

    for _ in 0..count as usize {
        let length = source.read_u8()? as usize;

        let mut path_bytes = vec![0u8; length];
        source.read_exact(&mut path_bytes)?;

        let mut path = String::from_utf8(path_bytes).map_err(|_| Error::Format)?;

        // I have no idea why original Fallout™ archives use . for root folder and
        // do not use .\ at start for all it's children, so...
        if path != "." { path = String::from(".\\") + &path; }

        let mut current: &mut Directory = &mut tree;
        let mut index_path = Vec::new();

        for (level, component) in path.split('\\').enumerate() {
            if level == 0 {
                index_path.push(None);
                continue;
            }

            match current.children
                .iter()
                .enumerate()
                .find(|n| n.1.name == component)
                .map(|v| v.0) {
                Some(existed) => {
                    index_path.push(Some(existed));
                    current = &mut current.children[existed];
                }
                None => {
                    current.children.push(Directory {
                        name: String::from(component),
                        files: Vec::new(),
                        children: Vec::new(),
                    });

                    index_path.push(Some(current.children.len() - 1));
                    current = current.children.last_mut().unwrap();
                }
            };
        }

        tree_paths.push(index_path);
    }

    for path in &tree_paths {
        let mut directory = &mut tree;
        for index in path.iter().flatten() {
            directory = &mut directory.children[*index];
        }

        let file_count = source.read_u32::<BigEndian>()? as usize;

        source.seek(SeekFrom::Current(3 * 4))?;

        for _ in 0..file_count {
            let length = source.read_u8()? as usize;

            let mut name_bytes = vec![u8::MIN; length];
            source.read_exact(&mut name_bytes)?;

            let name = String::from_utf8(name_bytes).map_err(|_| Error::Format)?;

            source.seek(SeekFrom::Current(4))?;

            let start = source.read_u32::<BigEndian>()?;
            let size = source.read_u32::<BigEndian>()?;
            let packed = source.read_u32::<BigEndian>()?;

            directory.files.push(File {
                name,
                range: start..start + { if packed > 0 { packed } else { size } },
                size,
            })
        }
    }

    Ok(Some(tree))
}
