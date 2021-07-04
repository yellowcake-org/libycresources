use libycresources::dat;

use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub(crate) enum Error {
    Path,
    Buffer,
    Decompress,
    Read(std::io::Error),
    Write(std::io::Error),
}

pub(crate) fn entry(input: &String, entries: &[dat::Entry], output: &String) -> Result<(), Error> {
    let mut file = match File::open(input) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let mut buffer: Vec<u8> = Vec::new();

    if let Err(error) = file.read_to_end(&mut buffer) {
        return Err(Error::Read(error));
    }

    let mut reader = |range: std::ops::Range<usize>| {
        let result: Result<Vec<u8>, ()> = Ok(buffer[range].to_vec());
        result
    };

    let limit: usize = 20 * 1024 * 1024;
    let mut buffer: Vec<u8> = Vec::new();

    for item in entries {
        buffer.clear();

        println!("Extracting {:?}...", item.path);

        let root = std::path::Path::new(&output);
        let joined = root.join(&item.path);
        let path = joined.as_path();

        let directory = match path.parent() {
            None => return Err(Error::Path),
            Some(directory) => directory,
        };

        if let Err(error) = std::fs::create_dir_all(&directory) {
            return Err(Error::Write(error));
        }

        let mut created = match std::fs::File::create(&path) {
            Err(error) => return Err(Error::Write(error)),
            Ok(created) => created,
        };

        let mut writer = |bytes: &[u8]| {
            buffer.extend_from_slice(bytes);

            if limit <= buffer.len() {
                if let Err(error) = created.write(buffer.as_slice()) {
                    return Err(error);
                }

                buffer.clear();
            }

            Ok(bytes.len())
        };

        if let Err(error) = dat::extract::entry(&mut reader, &item, &mut writer) {
            return match error {
                dat::extract::Error::Read(_) => Err(Error::Buffer),
                dat::extract::Error::Reader => Err(Error::Buffer),
                dat::extract::Error::Write(error) => Err(Error::Write(error)),
                dat::extract::Error::Decompress => Err(Error::Decompress),
            };
        }
    }

    Ok(())
}
