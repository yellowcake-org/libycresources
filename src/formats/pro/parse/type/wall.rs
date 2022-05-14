use std::collections::HashSet;

use crate::formats::pro::traits::TryFromOptional;

use super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::wall::Instance, errors::Error> {
    let mut light_bytes = [0u8; 2];
    match source.read_exact(&mut light_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light = match super::common::light::extract(light_bytes[0]) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format),
    };

    let mut actions_bytes = [0u8; 2];
    match source.read_exact(&mut actions_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let actions: HashSet<object::common::actions::Instance> =
        match super::common::actions::extract(actions_bytes[1]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format),
        };

    let script =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let mut material_id_bytes = [0u8; 4];
    match source.read_exact(&mut material_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let material = match object::common::world::Material::
    try_from(u32::from_be_bytes(material_id_bytes)) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format),
    };

    Ok(object::wall::Instance {
        light,
        script,
        material,
        actions,
    })
}