use libycresources::dat;

use std::fs::File;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};

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

    let limit: usize = 1 * 1024 * 1024;
    let mut buffered = 0..0;
    let mut buffer: Vec<u8> = Vec::new();

    let mut reader = |requested: std::ops::Range<usize>| {
        // TODO: Better buffering! Do not overwrite here
        if requested.start < buffered.start || requested.end > buffered.end {
            buffered = requested.start..std::cmp::max(requested.start + limit, requested.end);
            buffer.clear();

            if let Err(error) = file.seek(SeekFrom::Start(buffered.start as u64)) {
                return Err(error);
            }

            let size = buffered.end - buffered.start;
            let mut temp = vec![0u8; size];
            let read = match file.read(&mut temp) {
                Err(error) => return Err(error),
                Ok(value) => value,
            };

            // TODO: Additional checks on requested here!
            buffered.end -= size - read;
            buffer = temp[0..std::cmp::min(size, read)].to_vec();
        }

        Ok(buffer[(requested.start - buffered.start)..(requested.end - buffered.start)].to_vec())
    };

    for item in entries {
        let limit: usize = 1 * 1024 * 1024;

        let mut buffered: usize;
        let mut buffer: Vec<u8> = Vec::new();

        buffer.clear();
        buffered = 0;

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
            buffered += bytes.len();

            if limit <= buffered {
                if let Err(error) = created.write(buffer.as_slice()) {
                    return Err(error);
                }

                buffer.clear();
                buffered = 0;
            }

            Ok(bytes.len())
        };

        if let Err(error) = dat::extract::entry(&mut reader, &item, &mut writer) {
            return match error {
                dat::extract::Error::Read(error) => Err(Error::Read(error)),
                dat::extract::Error::Reader => Err(Error::Buffer),
                dat::extract::Error::Write(error) => Err(Error::Write(error)),
                dat::extract::Error::Decompress => Err(Error::Decompress),
            };
        }

        if 0 < buffered {
            if let Err(error) = created.write(buffer.as_slice()) {
                return Err(Error::Write(error));
            }
        }
    }

    Ok(())
}
