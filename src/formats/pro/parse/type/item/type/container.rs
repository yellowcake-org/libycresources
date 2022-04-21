use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::container::Instance, errors::Error> {
    let mut container_size_bytes = [0u8; 4];
    match source.read_exact(&mut container_size_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let container_size = u32::from_be_bytes(container_size_bytes);

    let mut container_flags_bytes = [0u8; 4];
    match source.read_exact(&mut container_flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut container_flags: HashSet<object::item::container::Flag> =
        HashSet::new();

    if (container_flags_bytes[3] & 0x01) == 0x01 {
        if !container_flags.insert(object::item::container::Flag::NoPickUp) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (container_flags_bytes[3] & 0x08) == 0x08 {
        if !container_flags.insert(object::item::container::Flag::MagicHands) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    Ok(object::item::container::Instance {
        size: container_size,
        flags: container_flags,
    })
}