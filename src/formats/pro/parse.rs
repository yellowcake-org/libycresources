use super::*;

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use std::collections::{HashMap, HashSet};

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Flags,
    }

    #[derive(Debug)]
    pub enum Error {
        Read(std::io::Error),
        Format(Format),
        Source,
    }
}

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let mut id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let r#type = id_bytes[0];
    let id = u32::from_be_bytes(match id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut text_id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut text_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let text_id = u32::from_be_bytes(match text_id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut sprite_id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sprite_id = u32::from_be_bytes(match sprite_id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut lradius_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut lradius_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lradius_id = u32::from_be_bytes(match lradius_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut lintensity_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut lintensity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lintensity = u32::from_be_bytes(match lintensity_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut flags_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let flags = u32::from_be_bytes(match flags_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut flagset: HashSet<meta::info::flags::Instance> = HashSet::new();

    match flags.trailing_zeros() {
        0x00000008 => {
            let _ = flagset.insert(meta::info::flags::Instance::Flat);
        }
        0x00000010 => {
            let _ = flagset.insert(meta::info::flags::Instance::NotBlocking);
        }
        0x00000800 => {
            let _ = flagset.insert(meta::info::flags::Instance::MultiHex);
        }
        0x00001000 => {
            let _ = flagset.insert(meta::info::flags::Instance::NotBordered);
        }
        0x20000000 => {
            let _ = flagset.insert(meta::info::flags::Instance::LightThrough);
        }
        0x80000000 => {
            let _ = flagset.insert(meta::info::flags::Instance::ShotThrough);
        }
        _ => {}
    }

    if flags.trailing_zeros() == 0x00008000 {
        flagset.insert(meta::info::flags::Instance::Transparency(None));
    } else if flags.trailing_zeros() == 0x00004000 {
        flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Red,
        )));
    } else if flags.trailing_zeros() == 0x00010000 {
        flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Wall,
        )));
    } else if flags.trailing_zeros() == 0x00020000 {
        flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Glass,
        )));
    } else if flags.trailing_zeros() == 0x00040000 {
        flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Steam,
        )));
    } else if flags.trailing_zeros() == 0x00080000 {
        flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Energy,
        )));
    } else {
        return Err(errors::Error::Format(errors::Format::Type));
    }

    match r#type {
        0 => {}
        1 => {}
        2 => {}
        3 => {}
        4 => {}
        5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Flags)),
    }

    Err(errors::Error::Source)
}
