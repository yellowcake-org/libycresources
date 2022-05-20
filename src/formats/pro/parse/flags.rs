use std::collections::HashSet;

use errors::Error;
use meta::info::flags::Root;

use super::*;

pub(crate) fn common<S: Read>(source: &mut S) -> Result<HashSet<Root>, Error> {
    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut flags: HashSet<Root> = HashSet::new();

    if (flags_bytes[3] & 0x08) == 0x08 {
        if !flags.insert(Root::Flat) { return Err(Error::Format); }
    }

    if (flags_bytes[3] & 0x10) == 0x10 {
        if !flags.insert(Root::NotBlocking) { return Err(Error::Format); }
    }

    if (flags_bytes[2] & 0x08) == 0x08 {
        if !flags.insert(Root::MultiHex) { return Err(Error::Format); }
    }

    if (flags_bytes[2] & 0x10) == 0x10 {
        if !flags.insert(Root::NotBordered) { return Err(Error::Format); }
    }

    if let Some(transparency) =
    if (flags_bytes[2] & 0x80) == 0x80 {
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
        if !flags.insert(Root::Transparency(transparency)) { return Err(Error::Format); }
    }

    if (flags_bytes[0] & 0x20) == 0x20 {
        if !flags.insert(Root::LightThrough) { return Err(Error::Format); }
    }

    if (flags_bytes[0] & 0x80) == 0x80 {
        if !flags.insert(Root::ShotThrough) { return Err(Error::Format); }
    }

    Ok(flags)
}

pub(crate) fn extended<S: Read>(source: &mut S) -> Result<HashSet<Root>, Error> {
    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut flags: HashSet<Root> = HashSet::new();

    if (flags_bytes[3] & 0x02) == 0x02 {
        if !flags.insert(Root::Locked) { return Err(Error::Format); }
    }

    if (flags_bytes[3] & 0x04) == 0x04 {
        if !flags.insert(Root::Jammed) { return Err(Error::Format); }
    }

    Ok(flags)
}