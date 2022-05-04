use std::collections::HashSet;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::container::Instance, errors::Error> {
    let mut size_bytes = [0u8; 4];
    match source.read_exact(&mut size_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let size = u32::from_be_bytes(size_bytes);

    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<object::item::container::Flag> = HashSet::new();

    if (flags_bytes[3] & 0x01) == 0x01 {
        if !flags.insert(object::item::container::Flag::NoPickUp) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x08) == 0x08 {
        if !flags.insert(object::item::container::Flag::MagicHands) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    Ok(object::item::container::Instance { size, flags })
}