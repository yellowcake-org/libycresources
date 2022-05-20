use std::collections::HashSet;

use super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<HashSet<meta::info::flags::Common>, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<meta::info::flags::Common> = HashSet::new();

    if (flags_bytes[3] & 0x08) == 0x08 {
        if !flags.insert(meta::info::flags::Common::Flat) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[3] & 0x10) == 0x10 {
        if !flags.insert(meta::info::flags::Common::NotBlocking) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[2] & 0x08) == 0x08 {
        if !flags.insert(meta::info::flags::Common::MultiHex) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[2] & 0x10) == 0x10 {
        if !flags.insert(meta::info::flags::Common::NotBordered) {
            return Err(errors::Error::Format);
        }
    }

    if let Some(transparency) = if (flags_bytes[2] & 0x80) == 0x80 {
        Some(None)
    } else if (flags_bytes[2] & 0x40) == 0x40 {
        Some(Some(meta::info::flags::Transparency::Red))
    } else if (flags_bytes[1] & 0x01) == 0x01 {
        Some(Some(meta::info::flags::Transparency::Wall))
    } else if (flags_bytes[1] & 0x02) == 0x02 {
        Some(Some(meta::info::flags::Transparency::Glass))
    } else if (flags_bytes[1] & 0x04) == 0x04 {
        Some(Some(meta::info::flags::Transparency::Steam))
    } else if (flags_bytes[1] & 0x08) == 0x08 {
        Some(Some(meta::info::flags::Transparency::Energy))
    } else if (flags_bytes[0] & 0x10) == 0x10 {
        Some(Some(meta::info::flags::Transparency::End))
    } else { None } {
        if !flags.insert(meta::info::flags::Common::Transparency(transparency)) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[0] & 0x20) == 0x20 {
        if !flags.insert(meta::info::flags::Common::LightThrough) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[0] & 0x80) == 0x80 {
        if !flags.insert(meta::info::flags::Common::ShotThrough) {
            return Err(errors::Error::Format);
        }
    }

    Ok(flags)
}