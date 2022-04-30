use std::collections::HashSet;

use crate::formats::pro::traits::TryFromOptional;

use super::super::*;

mod r#type;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::Instance, errors::Error> {
    let mut light_bytes = [0u8; 2];
    match source.read_exact(&mut light_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light = match super::common::light::extract(light_bytes[0]) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let mut actions_bytes = [0u8; 2];
    match source.read_exact(&mut actions_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let actions: HashSet<object::common::actions::Instance> =
        match super::common::actions::extract(actions_bytes[1]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    let mut script_id_bytes = [0u8; 4];
    match source.read_exact(&mut script_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let script =
        match object::common::script::Reference::
        try_from_optional(script_id_bytes, [0xFF, 0xFF, 0xFF, 0xFF]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    let mut type_id_bytes = [0u8; 4];
    match source.read_exact(&mut type_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let type_id = u32::from_be_bytes(type_id_bytes);

    let mut material_id_bytes = [0u8; 4];
    match source.read_exact(&mut material_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let material = match object::common::world::Material::
    try_from(u32::from_be_bytes(material_id_bytes)) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let mut sound_ids_bytes = [0u8; 1];
    match source.read_exact(&mut sound_ids_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sound_ids = u8::from_be_bytes(sound_ids_bytes);

    let r#type = match r#type::instance(source, type_id) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(object::scenery::Instance {
        r#type,
        light,
        script,
        material,
        actions,
        connections: object::scenery::Connections {
            _sounds_ids: sound_ids
        },
    })
}