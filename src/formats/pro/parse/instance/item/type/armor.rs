use std::collections::HashMap;

use object::common::combat::damage;
use object::common::critter::Gender;
use object::item::armor::Instance;

use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let ac = source.read_u32::<BigEndian>()?;

    fn damage<S: Read>(source: &mut S) -> Result<HashMap<damage::Type, u32>, errors::Error> {
        Ok(HashMap::from([
            (damage::Type::Default, source.read_u32::<BigEndian>()?),
            (damage::Type::Laser, source.read_u32::<BigEndian>()?),
            (damage::Type::Fire, source.read_u32::<BigEndian>()?),
            (damage::Type::Plasma, source.read_u32::<BigEndian>()?),
            (damage::Type::Electrical, source.read_u32::<BigEndian>()?),
            (damage::Type::Emp, source.read_u32::<BigEndian>()?),
            (damage::Type::Explosive, source.read_u32::<BigEndian>()?),
        ]))
    }

    let resistance = damage(source)?;
    let threshold = damage(source)?;

    let perk_raw = source.read_i32::<BigEndian>()?;

    let male_sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let female_sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    Ok(Instance {
        class: ac,
        threshold,
        resistance,
        perk: object::common::critter::Perk::try_from_optional(perk_raw, -1)?,
        appearance: object::item::armor::Appearance {
            sprites: HashMap::from([
                (Gender::Male, male_sprite),
                (Gender::Female, female_sprite)
            ])
        },
    })
}