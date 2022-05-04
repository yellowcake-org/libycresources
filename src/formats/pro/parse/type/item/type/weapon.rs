use std::collections::HashSet;

use super::super::super::*;
use super::super::super::traits::*;

pub(crate) fn instance<S: Read>(source: &mut S,
                                flags: HashSet<object::item::weapon::Flag>,
                                attack_modes: u8) -> Result<object::item::weapon::Instance, errors::Error> {
    let attack1_mode_raw = attack_modes & 0xf;
    let attack2_mode_raw = (attack_modes >> 4) & 0xf;

    let mut animation_bytes = [0u8; 4];
    match source.read_exact(&mut animation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let animation_raw = u32::from_be_bytes(animation_bytes);

    let animation = match animation_raw {
        0x00 => None,
        value => Some(match object::item::weapon::Animation::try_from(value) {
            Err(_) => return Err(errors::Error::Source),
            Ok(value) => value,
        })
    };

    let mut min_dmg_bytes = [0u8; 4];
    match source.read_exact(&mut min_dmg_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let min_dmg = u32::from_be_bytes(min_dmg_bytes);

    let mut max_dmg_bytes = [0u8; 4];
    match source.read_exact(&mut max_dmg_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let max_dmg = u32::from_be_bytes(max_dmg_bytes);

    let mut dmg_type_bytes = [0u8; 4];
    match source.read_exact(&mut dmg_type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dmg_type_raw = u32::from_be_bytes(dmg_type_bytes);

    let dmg_type =
        match object::common::combat::damage::Type::try_from(dmg_type_raw as u8) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    let damage = object::item::weapon::Damage {
        value: min_dmg..=max_dmg,
        r#type: dmg_type,
    };

    let mut dmg_range_max1_bytes = [0u8; 4];
    match source.read_exact(&mut dmg_range_max1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dmg_range_max1 = u32::from_be_bytes(dmg_range_max1_bytes);

    let mut dmg_range_max2_bytes = [0u8; 4];
    match source.read_exact(&mut dmg_range_max2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dmg_range_max2 = u32::from_be_bytes(dmg_range_max2_bytes);

    let mut projectile_header_bytes = [0u8; 2];
    match source.read_exact(&mut projectile_header_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let projectile_header = u16::from_be_bytes(projectile_header_bytes);

    let mut projectile_idx_bytes = [0u8; 2];
    match source.read_exact(&mut projectile_idx_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let projectile_idx = u16::from_be_bytes(projectile_idx_bytes);

    if 0xFFFF != projectile_idx && 0x0500 != projectile_header {
        return Err(errors::Error::Format(errors::Format::Consistency));
    }

    let mut min_strength_bytes = [0u8; 4];
    match source.read_exact(&mut min_strength_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let min_strength = u32::from_be_bytes(min_strength_bytes);

    let mut cost1_bytes = [0u8; 4];
    match source.read_exact(&mut cost1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let cost1 = u32::from_be_bytes(cost1_bytes);

    let mut cost2_bytes = [0u8; 4];
    match source.read_exact(&mut cost2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let cost2 = u32::from_be_bytes(cost2_bytes);

    let attack1 = match object::item::weapon::attack::Mode::
    try_from_optional(attack1_mode_raw, 0) {
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        Ok(value) => value.map(|mode| {
            object::item::weapon::attack::Instance {
                cost: cost1,
                mode,
                range: 0..=dmg_range_max1,
            }
        }),
    };

    let attack2 = match object::item::weapon::attack::Mode::
    try_from_optional(attack2_mode_raw, 0) {
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        Ok(value) => value.map(|mode| {
            object::item::weapon::attack::Instance {
                cost: cost2,
                mode,
                range: 0..=dmg_range_max2,
            }
        }),
    };

    let mut crit_list_idx_bytes = [0u8; 4];
    match source.read_exact(&mut crit_list_idx_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let crit_list_idx =
        match u16::try_from_optional(i32::from_be_bytes(crit_list_idx_bytes), -1) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Consistency)),
        };

    let mut perk_bytes = [0u8; 4];
    match source.read_exact(&mut perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let perk_raw = i32::from_be_bytes(perk_bytes);

    let perk = match object::common::critter::Perk::try_from_optional(perk_raw, -1) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut burst_bytes = [0u8; 4];
    match source.read_exact(&mut burst_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let burst_count = u32::from_be_bytes(burst_bytes);

    let mut caliber_bytes = [0u8; 4];
    match source.read_exact(&mut caliber_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let caliber_raw = u32::from_be_bytes(caliber_bytes);
    let caliber = match object::common::weapons::Caliber::try_from_optional(caliber_raw, 0) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut ammo_pid_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_pid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_pid =
        match u16::try_from_optional(i32::from_be_bytes(ammo_pid_bytes), -1) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Consistency)),
        };

    let mut capacity_bytes = [0u8; 4];
    match source.read_exact(&mut capacity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let capacity = u32::from_be_bytes(capacity_bytes);

    let mut sound_ids_bytes = [0u8; 1];
    match source.read_exact(&mut sound_ids_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sound_ids = u8::from_be_bytes(sound_ids_bytes);

    Ok(object::item::weapon::Instance {
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