use std::collections::HashSet;
use object::item::container::Instance;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let size = source.read_u32::<BigEndian>()?;

    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut flags: HashSet<object::item::container::Flag> = HashSet::new();

    if (flags_bytes[3] & 0x01) == 0x01 {
        if !flags.insert(object::item::container::Flag::NoPickUp) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[3] & 0x08) == 0x08 {
        if !flags.insert(object::item::container::Flag::MagicHands) {
            return Err(errors::Error::Format);
        }
    }

    Ok(Instance { size, flags })
}