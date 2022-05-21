use std::collections::HashSet;

use object::item::Instance;

use crate::common::traits::TryFromOptional;

use super::common::actions;
use super::super::*;

mod r#type;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut flags_bytes = [0u8; 3];
    source.read_exact(&mut flags_bytes)?;

    let mut flags: HashSet<object::item::Flag> = HashSet::new();
    let mut weapon_flags: HashSet<object::item::weapon::Flag> = HashSet::new();
    let actions: HashSet<object::common::actions::Instance> = actions::extract(flags_bytes[2])?;

    // Flags

    if (flags_bytes[0] & 0x08) == 0x08 && !flags.insert(object::item::Flag::Hidden) {
        return Err(errors::Error::Format);
    }

    // Weapon Flags

    if (flags_bytes[2] & 0x01) == 0x01 && !weapon_flags.insert(object::item::weapon::Flag::BigGun) {
        return Err(errors::Error::Format);
    }

    if (flags_bytes[2] & 0x02) == 0x02 && !weapon_flags.insert(object::item::weapon::Flag::SecondHand) {
        return Err(errors::Error::Format);
    }

    let attack_modes = source.read_u8()?;

    let script = Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let type_id = source.read_u32::<BigEndian>()?;
    let material_id = source.read_u32::<BigEndian>()?;

    let size = source.read_u32::<BigEndian>()?;
    let weight = source.read_u32::<BigEndian>()?;
    let cost = source.read_u32::<BigEndian>()?;

    let sprite =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let sound_ids = source.read_u8()?;
    let r#type = r#type::instance(source, type_id, weapon_flags, attack_modes)?;

    Ok(Instance {
        r#type,
        flags,
        sprite,
        script,
        actions,
        material: object::common::world::Material::try_from(material_id)?,
        size,
        price: cost,
        weight,
        connections: object::item::Connections {
            _sounds_ids: sound_ids,
        },
    })
}