use libycresources::dat;

use std::fs::File;
use std::io::Read;
use std::io::{Seek, SeekFrom};

#[derive(Debug)]
pub(crate) enum Error {
    Buffer,
    Corrupted,
    Read(std::io::Error),
}

pub(crate) fn entries(input: &String) -> Result<Vec<dat::Entry>, Error> {
    let mut file = match File::open(input) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let mut reader = |range: std::ops::Range<usize>| {
        if let Err(error) = file.seek(SeekFrom::Start(range.start as u64)) {
            return Err(error);
        }

        let mut bytes = vec![0u8; range.end - range.start];

        if let Err(error) = file.read_exact(&mut bytes) {
            return Err(error);
        }

        Ok(bytes)
    };

    dat::list::entries(&mut reader).map_err(|e| match e {
        dat::list::Error::Format => Error::Corrupted,
        dat::list::Error::Reader => Error::Buffer,
        dat::list::Error::Read(_) => Error::Buffer,
    })
}
