use super::fetch;

use super::Entry;
use super::Error;

use std::fs::File;
use std::io::Seek;

use std::str::FromStr;

pub fn entries(mut file: &File) -> Result<Vec<Entry>, Error> {
    let count = match fetch::u32(file, Some(0)) {
        Err(error) => return Err(error),
        Ok(value) => value,
    };

    if let Err(error) = file.seek(std::io::SeekFrom::Start(4 * 4_u64)) {
        return Err(Error::Read(error));
    }

    let mut dirs = Vec::new();

    for _ in 0..count {
        let mut name: String = match fetch::string(file, None) {
            Err(error) => return Err(error),
            Ok(value) => value,
        }
        .split('\\')
        .map(|dir| dir.to_owned() + "/")
        .collect();

        name.pop();
        dirs.push(name);
    }

    let mut entries = Vec::new();

    for dir in &dirs {
        let file_count = match fetch::u32(file, None) {
            Err(error) => return Err(error),
            Ok(value) => value,
        };

        if let Err(error) = file.seek(std::io::SeekFrom::Current(3 * 4_i64)) {
            return Err(Error::Read(error));
        }

        for _ in 0..file_count {
            let name = match fetch::string(file, None) {
                Err(error) => return Err(error),
                Ok(value) => value,
            };

            let path = String::from_str(&dir).unwrap() + "/" + &name;

            if let Err(error) = file.seek(std::io::SeekFrom::Current(4)) {
                return Err(Error::Read(error));
            }

            let offset = match fetch::u32(file, None) {
                Err(error) => return Err(error),
                Ok(value) => value,
            } as usize;

            let size = match fetch::u32(file, None) {
                Err(error) => return Err(error),
                Ok(value) => value,
            } as usize;

            let packed_size = match fetch::u32(file, None) {
                Err(error) => return Err(error),
                Ok(value) => value,
            } as usize;

            entries.push(Entry {
                path,
                range: offset..offset + {
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
