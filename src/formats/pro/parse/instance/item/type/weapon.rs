use std::collections::HashSet;

use object::common::combat::damage;
use object::common::critter::Perk;
use object::common::weapons::Caliber;
use object::item::weapon::{Animation, attack, Damage, Instance};

use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S,
                                flags: HashSet<object::item::weapon::Flag>,
                                attack_modes: u8) -> Result<Instance, errors::Error> {
    let attack1_mode_raw = attack_modes & 0xf;
    let attack2_mode_raw = (attack_modes >> 4) & 0xf;

    let animation = Animation::try_from_optional(source.read_u32::<BigEndian>()?, 0x00)?;

    let damage = Damage {
        value: source.read_u32::<BigEndian>()?..=source.read_u32::<BigEndian>()?,
        r#type: damage::Type::try_from(source.read_u32::<BigEndian>()? as u8)?,
    };


    let dmg_range_max1 = source.read_u32::<BigEndian>()?;
    let dmg_range_max2 = source.read_u32::<BigEndian>()?;

    let projectile_header = source.read_u16::<BigEndian>()?;
    let projectile_idx = source.read_u16::<BigEndian>()?;

    if 0xFFFF != projectile_idx && 0x0500 != projectile_header { return Err(errors::Error::Format); }


    let min_strength = source.read_u32::<BigEndian>()?;
    let cost1 = source.read_u32::<BigEndian>()?;
    let cost2 = source.read_u32::<BigEndian>()?;

    fn attack(cost: u32, rng: u32, mode: u8) -> Result<Option<attack::Instance>, errors::Error> {
        Ok(attack::Mode::try_from_optional(mode, 0)
            .map_err(|_| errors::Error::Format)?
            .map_or(None, |mode| {
                Some(attack::Instance {
                    cost,
                    mode,
                    range: 0..=rng,
                })
            }))
    }

    let attack1 = attack(cost1, dmg_range_max1, attack1_mode_raw)?;
    let attack2 = attack(cost2, dmg_range_max2, attack2_mode_raw)?;

    let crit_list_idx = u16::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    let perk = Perk::try_from_optional(source.read_i32::<BigEndian>()?, -1)?;

    let burst_count = source.read_u32::<BigEndian>()?;
    let caliber = Caliber::try_from_optional(source.read_u32::<BigEndian>()?, 0)?;

    let ammo_pid = u16::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    let capacity = source.read_u32::<BigEndian>()?;
    let sound_ids = source.read_u8()?;

    Ok(Instance {
        flags,
        damage,
        attacks: [attack1, attack2],
        animation,
        requirements: object::item::weapon::Requirements {
            strength: min_strength
        },
        ammunition: caliber.map(|caliber| {
            object::item::weapon::Ammunition {
                rounds: object::item::weapon::Rounds {
                    burst: burst_count,
                    magazine: capacity,
                },
                caliber,
            }
        }),
        perk,
        connections: object::item::weapon::Connections {
            ammo_item_id: ammo_pid,
            failure_list_id: crit_list_idx,
            projectile_misc_id: if projectile_idx != 0xFFFF { Some(projectile_idx) } else { None },
            _sounds_ids: sound_ids,
        },
    })
}