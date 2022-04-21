use super::super::*;
use super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<HashSet<meta::info::flags::Instance>, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<meta::info::flags::Instance> = HashSet::new();

    if (flags_bytes[0] & 0x08) == 0x08 {
        if !flags.insert(meta::info::flags::Instance::Flat) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[0] & 0x10) == 0x10 {
        if !flags.insert(meta::info::flags::Instance::NotBlocking) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x08) == 0x08 {
        if !flags.insert(meta::info::flags::Instance::MultiHex) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x10) == 0x10 {
        if !flags.insert(meta::info::flags::Instance::NotBordered) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x20) == 0x20 {
        if !flags.insert(meta::info::flags::Instance::LightThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x80) == 0x80 {
        if !flags.insert(meta::info::flags::Instance::ShotThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if !flags.insert(meta::info::flags::Instance::Transparency(
        if (flags_bytes[1] & 0x80) == 0x80 {
            None
        } else if (flags_bytes[1] & 0x40) == 0x40 {
            Some(meta::info::flags::Transparency::Red)
        } else if (flags_bytes[2] & 0x01) == 0x01 {
            Some(meta::info::flags::Transparency::Wall)
        } else if (flags_bytes[2] & 0x02) == 0x02 {
            Some(meta::info::flags::Transparency::Glass)
        } else if (flags_bytes[2] & 0x04) == 0x04 {
            Some(meta::info::flags::Transparency::Steam)
        } else if (flags_bytes[2] & 0x08) == 0x08 {
            Some(meta::info::flags::Transparency::Energy)
        } else {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    )) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    Ok(flags)
}