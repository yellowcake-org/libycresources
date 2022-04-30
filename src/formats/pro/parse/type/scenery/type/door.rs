use std::collections::HashSet;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::door::Instance, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<object::scenery::door::Flags> = HashSet::new();
    if (flags_bytes[3] & 0x0F) == 0x0F &&
        !flags.insert(object::scenery::door::Flags::Passable) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    let mut unknown_bytes = [0u8; 4];
    match source.read_exact(&mut unknown_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    Ok(object::scenery::door::Instance { flags, _unknown: u32::from_be_bytes(unknown_bytes) })
}