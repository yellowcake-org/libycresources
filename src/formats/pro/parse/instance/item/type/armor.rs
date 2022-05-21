use std::collections::HashMap;

use object::common::combat::damage;
use object::common::critter::Gender;
use object::item::armor::Instance;

use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let ac = source.read_u32::<BigEndian>()?;

    let dr_normal = source.read_u32::<BigEndian>()?;
    let dr_laser = source.read_u32::<BigEndian>()?;
    let dr_fire = source.read_u32::<BigEndian>()?;
    let dr_plasma = source.read_u32::<BigEndian>()?;
    let dr_electrical = source.read_u32::<BigEndian>()?;
    let dr_emp = source.read_u32::<BigEndian>()?;
    let dr_explosive = source.read_u32::<BigEndian>()?;

    let dt_normal = source.read_u32::<BigEndian>()?;
    let dt_laser = source.read_u32::<BigEndian>()?;
    let dt_fire = source.read_u32::<BigEndian>()?;
    let dt_plasma = source.read_u32::<BigEndian>()?;
    let dt_electrical = source.read_u32::<BigEndian>()?;
    let dt_emp = source.read_u32::<BigEndian>()?;
    let dt_explosive = source.read_u32::<BigEndian>()?;

    let perk_raw = source.read_i32::<BigEndian>()?;

    let male_sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let female_sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    let result = Ok(Instance {
        class: ac,
        threshold: HashMap::from([
            (damage::Type::Default, dt_normal),
            (damage::Type::Laser, dt_laser),
            (damage::Type::Fire, dt_fire),
            (damage::Type::Plasma, dt_plasma),
            (damage::Type::Electrical, dt_electrical),
            (damage::Type::Emp, dt_emp),
            (damage::Type::Explosive, dt_explosive),
        ]),
        resistance: HashMap::from([
            (damage::Type::Default, dr_normal),
            (damage::Type::Laser, dr_laser),
            (damage::Type::Fire, dr_fire),
            (damage::Type::Plasma, dr_plasma),
            (damage::Type::Electrical, dr_electrical),
            (damage::Type::Emp, dr_emp),
            (damage::Type::Explosive, dr_explosive),
        ]),
        perk: object::common::critter::Perk::try_from_optional(perk_raw, -1)?,
        appearance: object::item::armor::Appearance {
            sprites: HashMap::from([
                (Gender::Male, male_sprite),
                (Gender::Female, female_sprite)
            ])
        },
    });
    result
}