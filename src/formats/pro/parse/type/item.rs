use std::collections::HashSet;

use super::super::*;
use super::super::traits::TryFromOptional;

mod r#type;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::Instance, errors::Error> {
    let mut flags_bytes = [0u8; 3];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<object::item::Flag> = HashSet::new();
    let mut weapon_flags: HashSet<object::item::weapon::Flag> = HashSet::new();
    let actions: HashSet<object::common::actions::Instance> =
        match super::common::actions::extract(flags_bytes[2]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    // Flags

    if (flags_bytes[0] & 0x08) == 0x08 &&
        !flags.insert(object::item::Flag::Hidden) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    // Weapon Flags

    if (flags_bytes[2] & 0x01) == 0x01 &&
        !weapon_flags.insert(object::item::weapon::Flag::BigGun) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x02) == 0x02 &&
        !weapon_flags.insert(object::item::weapon::Flag::SecondHand) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    let mut attack_modes_bytes = [0u8; 1];
    match source.read_exact(&mut attack_modes_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let attack_modes = u8::from_be_bytes(attack_modes_bytes);

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

    let mut type_bytes = [0u8; 4];
    match source.read_exact(&mut type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let type_id = u32::from_be_bytes(type_bytes);

    let mut material_id_bytes = [0u8; 4];
    match source.read_exact(&mut material_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let material_id = u32::from_be_bytes(material_id_bytes);

    let mut size_bytes = [0u8; 4];
    match source.read_exact(&mut size_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let size = u32::from_be_bytes(size_bytes);

    let mut weight_bytes = [0u8; 4];
    match source.read_exact(&mut weight_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weight = u32::from_be_bytes(weight_bytes);

    let mut cost_bytes = [0u8; 4];
    match source.read_exact(&mut cost_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let cost = u32::from_be_bytes(cost_bytes);

    let mut sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut sound_ids_bytes = [0u8; 1];
    match source.read_exact(&mut sound_ids_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sound_ids = u8::from_be_bytes(sound_ids_bytes);
    let r#type = match r#type::instance(source, type_id, weapon_flags, attack_modes) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(object::item::Instance {
        r#type,
        flags,
        sprite: match object::common::sprite::Reference::try_from(sprite_id_bytes) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        },
        script,
        actions,
        material: match object::common::world::Material::try_from(material_id) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
        },
        size,
        price: cost,
        weight,
        connections: object::item::Connections {
            _sounds_ids: sound_ids,
        },
    })
}