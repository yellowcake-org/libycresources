use object::item::ammo::{adjustments, Instance};
use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let caliber_raw = source.read_u32::<BigEndian>()?;
    let count = source.read_u32::<BigEndian>()?;


    let ac_modifier = source.read_i32::<BigEndian>()?;
    let dr_modifier = source.read_i32::<BigEndian>()?;

    let dmg_multiplier = source.read_u32::<BigEndian>()?;
    let dmg_divider = source.read_u32::<BigEndian>()?;

    Ok(Instance {
        count,
        caliber: object::common::weapons::Caliber::try_from_optional(caliber_raw, 0)?,
        adjustments: adjustments::Instance {
            armor: adjustments::Armor {
                class: ac_modifier,
                resistance: dr_modifier,
            },
            damage: adjustments::Damage {
                divider: dmg_divider,
                multiplier: dmg_multiplier,
            },
        },
    })
}