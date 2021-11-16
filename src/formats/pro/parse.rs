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

    let mut flagset: HashSet<meta::info::flags::Instance> = HashSet::new();

    if (flags_bytes[0] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::Flat) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[0] & 0x10) == 0x10 {
        if !flagset.insert(meta::info::flags::Instance::NotBlocking) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::MultiHex) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x10) == 0x10 {
        if !flagset.insert(meta::info::flags::Instance::NotBordered) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x20) == 0x20 {
        if !flagset.insert(meta::info::flags::Instance::LightThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x80) == 0x80 {
        if !flagset.insert(meta::info::flags::Instance::ShotThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x80) == 0x80 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(None)) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[1] & 0x40) == 0x40 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Red,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x01) == 0x01 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Wall,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x02) == 0x02 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Glass,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x04) == 0x04 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Steam,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Energy,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    match r#type {
        0 => {
            let mut item_flags_bytes = vec![0u8; 3];
            match source.read_exact(&mut item_flags_bytes[1..=3]) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_is_hidden = (item_flags_bytes[2] & 0x08) == 0x08;
            let mut item_flagset: HashSet<meta::info::flags::Instance> = HashSet::new();
        }
        1 => {}
        2 => {}
        3 => {}
        4 => {}
        5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    }

    Err(errors::Error::Source)
}
