use std::collections::HashSet;

use object::scenery::Instance;

use crate::common::traits::TryFromOptional;

use super::common::actions;
use super::super::*;

mod r#type;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut light_bytes = [0u8; 2];
    source.read_exact(&mut light_bytes)?;

    let light = super::common::light::extract(light_bytes[0])?;

    let mut actions_bytes = [0u8; 2];
    source.read_exact(&mut actions_bytes)?;

    let actions: HashSet<object::common::actions::Instance> = actions::extract(actions_bytes[1])?;

    let script =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let type_id = source.read_u32::<BigEndian>()?;
    let material = object::common::world::Material::try_from(source.read_u32::<BigEndian>()?)?;
    let sound_ids = source.read_u8()?;
    let body = r#type::body(source, type_id)?;

    Ok(Instance {
        body,
        light,
        script,
        material,
        actions,
        connections: object::scenery::Connections {
            _sounds_ids: sound_ids
        },
    })
}