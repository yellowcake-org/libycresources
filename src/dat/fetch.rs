use super::Error;

use std::fs::File;
use std::io::Read;
use std::io::Seek;

pub(super) fn u32(mut file: &File, offset: Option<u64>) -> Result<u32, Error> {
    if let Err(error) = self::offset(file, offset) {
        return Err(error);
    }

    const COUNT: usize = std::mem::size_of::<u32>();
    let mut slice: [u8; COUNT] = [0; COUNT];

    match file.read_exact(&mut slice) {
        Err(error) => Err(Error::Read(error)),
        Ok(_) => Ok(u32::from_be_bytes(slice)),
    }
}

pub(super) fn i16(mut file: &File, offset: Option<u64>) -> Result<i16, Error> {
    if let Err(error) = self::offset(file, offset) {
        return Err(error);
    }

    const COUNT: usize = std::mem::size_of::<i16>();
    let mut slice: [u8; COUNT] = [0; COUNT];

    match file.read_exact(&mut slice) {
        Err(error) => Err(Error::Read(error)),
        Ok(_) => Ok(i16::from_be_bytes(slice)),
    }
}

pub(super) fn u8(mut file: &File, offset: Option<u64>) -> Result<u8, Error> {
    if let Err(error) = self::offset(file, offset) {
        return Err(error);
    }

    const COUNT: usize = std::mem::size_of::<u8>();
    let mut slice: [u8; COUNT] = [0; COUNT];

    match file.read_exact(&mut slice) {
        Err(error) => Err(Error::Read(error)),
        Ok(_) => Ok(u8::from_be_bytes(slice)),
    }
}

pub(super) fn string(mut file: &File, offset: Option<u64>) -> Result<String, Error> {
    let string_length = match u8(file, offset) {
        Err(error) => return Err(error),
        Ok(value) => value,
    };

    let mut string_slice = vec![0u8; string_length as usize];
    if let Err(error) = file.read_exact(&mut string_slice) {
        return Err(Error::Read(error));
    }

    match String::from_utf8(string_slice) {
        Err(error) => Err(Error::Decode(error)),
        Ok(string) => Ok(string),
    }
}

fn offset(mut file: &File, offset: Option<u64>) -> Result<(), Error> {
    if let Some(offset) = offset {
        if let Err(error) = file.seek(std::io::SeekFrom::Start(offset)) {
            return Err(Error::Read(error));
        }
    }

    Ok(())
}
