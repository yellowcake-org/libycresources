use std::collections::HashSet;

use object::common::world::Material;
use object::wall::Instance;

use crate::common::traits::TryFromOptional;
use crate::formats::pro::parse::instance::common::actions;

use super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut light_bytes = [0u8; 2];
    source.read_exact(&mut light_bytes)?;

    let light = super::common::light::extract(light_bytes[0])?;

    let mut actions_bytes = [0u8; 2];
    source.read_exact(&mut actions_bytes)?;

    let actions: HashSet<object::common::actions::Instance> = actions::extract(actions_bytes[1])?;

    let script =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let material = Material::try_from(source.read_u32::<BigEndian>()?)?;

    Ok(Instance {
        light,
        script,
        material,
        actions,
    })
}