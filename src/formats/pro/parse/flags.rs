use super::super::*;
use super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<HashSet<meta::info::flags::Instance>, errors::Error> {
    let mut flags_bytes = [0u8; 4];
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

    Ok(flagset)
}