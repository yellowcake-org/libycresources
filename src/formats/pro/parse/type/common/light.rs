use std::collections::HashSet;

use super::super::super::*;

pub(crate) fn extract(from: u8) -> Result<HashSet<object::common::world::Light>, errors::Error> {
    let mut light: HashSet<object::common::world::Light> = HashSet::new();

    if (from & 0x00) == 0x00 &&
        !light.insert(object::common::world::Light::Vertical) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (from & 0x08) == 0x08 &&
        !light.insert(object::common::world::Light::Horizontal) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (from & 0x10) == 0x10 &&
        !light.insert(object::common::world::Light::NorthCorner) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (from & 0x20) == 0x20 &&
        !light.insert(object::common::world::Light::SouthCorner) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (from & 0x40) == 0x40 &&
        !light.insert(object::common::world::Light::EastCorner) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (from & 0x80) == 0x80 &&
        !light.insert(object::common::world::Light::WestCorner) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    Ok(light)
}