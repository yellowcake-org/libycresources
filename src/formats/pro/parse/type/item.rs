use super::super::*;

mod r#type;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::Instance, errors::Error> {
    let mut item_flags_bytes = [0u8; 3];
    match source.read_exact(&mut item_flags_bytes[1..=3]) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut item_flags: HashSet<object::item::Flag> = HashSet::new();
    let mut weapon_flags: HashSet<object::item::weapon::Flag> = HashSet::new();
    let mut item_actions: HashSet<object::common::actions::Instance> = HashSet::new();

    if (item_flags_bytes[0] & 0x01) == 0x01 {
        if !weapon_flags.insert(object::item::weapon::Flag::BigGun) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (item_flags_bytes[0] & 0x02) == 0x02 {
        if !weapon_flags.insert(object::item::weapon::Flag::SecondHand) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (item_flags_bytes[0] & 0x80) == 0x80 {
        if !item_actions.insert(object::common::actions::Instance::PickUp) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (item_flags_bytes[2] & 0x08) == 0x08 {
        if !item_flags.insert(object::item::Flag::Hidden) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    let can_use = (item_flags_bytes[0] & 0x08) == 0x08;
    let can_use_on = (item_flags_bytes[0] & 0x10) == 0x10;

    let usage = object::common::actions::Usage {
        itself: can_use,
        something: can_use_on,
        knees_down: false,
    };

    if !item_actions.insert(object::common::actions::Instance::Usage(usage)) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    let mut attack_modes_bytes = [0u8; 1];
    match source.read_exact(&mut attack_modes_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let attack_modes = u8::from_be_bytes(attack_modes_bytes);

    let mut item_script_id_bytes = [0u8; 4];
    match source.read_exact(&mut item_script_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut item_type_bytes = [0u8; 4];
    match source.read_exact(&mut item_type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_type_id = u32::from_be_bytes(item_type_bytes);

    let mut material_id_bytes = [0u8; 4];
    match source.read_exact(&mut material_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let material_id = u32::from_be_bytes(material_id_bytes);

    let mut item_size_bytes = [0u8; 4];
    match source.read_exact(&mut item_size_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_size = u32::from_be_bytes(item_size_bytes);

    let mut item_weight_bytes = [0u8; 4];
    match source.read_exact(&mut item_weight_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_weight = u32::from_be_bytes(item_weight_bytes);

    let mut item_cost_bytes = [0u8; 4];
    match source.read_exact(&mut item_cost_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_cost = u32::from_be_bytes(item_cost_bytes);

    let mut item_sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut item_sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut item_sound_ids_bytes = [0u8; 1];
    match source.read_exact(&mut item_sound_ids_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_sound_ids = u8::from_be_bytes(item_sound_ids_bytes);

    Ok(object::item::Instance {
        r#type: match r#type::instance(source, item_type_id, weapon_flags, attack_modes) {
            Ok(value) => value,
            Err(error) => return Err(error)
        },
        flags: item_flags,
        sprite: match object::common::sprite::Reference::try_from(item_sprite_id_bytes) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        },
        script: match item_script_id_bytes {
            [0xFF, 0xFF, 0xFF, 0xFF] => None,
            value => match object::common::script::Reference::try_from(value) {
                Ok(value) => Some(value),
                Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
            }
        },
        actions: item_actions,
        material: match object::common::world::Material::try_from(material_id) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
        },
        size: item_size,
        price: item_cost,
        weight: item_weight,
        connections: object::item::Connections {
            _sounds_ids: item_sound_ids,
        },
    })
}