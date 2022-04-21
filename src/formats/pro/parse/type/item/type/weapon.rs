use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S,
                                weapon_flags: HashSet<object::item::weapon::Flag>,
                                attack_modes: u8) -> Result<object::item::weapon::Instance, errors::Error> {
    let attack_mode_primary_raw = attack_modes & 0xf;
    let attack_mode_secondary_raw = (attack_modes >> 4) & 0xf;

    let attack_mode_primary =
        match attack_mode_primary_raw {
            0 => None,
            value => Some(
                match object::item::weapon::attack::Mode::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let attack_mode_secondary =
        match attack_mode_secondary_raw {
            0 => None,
            value => Some(
                match object::item::weapon::attack::Mode::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let mut weapon_animation_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_animation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_animation_raw = u32::from_be_bytes(weapon_animation_bytes);

    let weapon_animation = match weapon_animation_raw {
        0x00 => None,
        value => Some(
            match object::item::weapon::Animation::try_from(value) {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            }
        )
    };

    let mut weapon_min_dmg_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_min_dmg_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_min_dmg = u32::from_be_bytes(weapon_min_dmg_bytes);

    let mut weapon_max_dmg_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_max_dmg_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_max_dmg = u32::from_be_bytes(weapon_max_dmg_bytes);

    let mut weapon_dmg_type_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_dmg_type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_dmg_type_raw = u32::from_be_bytes(weapon_dmg_type_bytes);

    let weapon_dmg_type = match object::common::combat::damage::Type::try_from(
        weapon_dmg_type_raw as u8
    ) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let weapon_damage = object::item::weapon::Damage {
        value: weapon_min_dmg..=weapon_max_dmg,
        r#type: weapon_dmg_type,
    };

    let mut weapon_dmg_range_max1_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_dmg_range_max1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_dmg_range_max1 = u32::from_be_bytes(weapon_dmg_range_max1_bytes);

    let mut weapon_dmg_range_max2_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_dmg_range_max2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_dmg_range_max2 = u32::from_be_bytes(weapon_dmg_range_max2_bytes);

    let mut weapon_projectile_header_bytes = [0u8; 2];
    match source.read_exact(&mut weapon_projectile_header_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_projectile_header = u16::from_be_bytes(weapon_projectile_header_bytes);

    if 0x0500 != weapon_projectile_header {
        return Err(errors::Error::Format(errors::Format::Consistency));
    }

    let mut weapon_projectile_idx_bytes = [0u8; 2];
    match source.read_exact(&mut weapon_projectile_idx_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_projectile_idx = u16::from_be_bytes(weapon_projectile_idx_bytes);

    let mut weapon_min_strength_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_min_strength_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_min_strength = u32::from_be_bytes(weapon_min_strength_bytes);

    let mut weapon_cost1_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_cost1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_cost1 = u32::from_be_bytes(weapon_cost1_bytes);

    let mut weapon_cost2_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_cost2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_cost2 = u32::from_be_bytes(weapon_cost2_bytes);

    let weapon_attack1 = object::item::weapon::attack::Instance {
        cost: weapon_cost1,
        mode: attack_mode_primary,
        range: 0..=weapon_dmg_range_max1,
    };

    let weapon_attack2 = object::item::weapon::attack::Instance {
        cost: weapon_cost2,
        mode: attack_mode_secondary,
        range: 0..=weapon_dmg_range_max2,
    };

    let mut weapon_crit_list_idx_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_crit_list_idx_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_crit_list_idx = u32::from_be_bytes(weapon_crit_list_idx_bytes);

    let mut weapon_perk_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_perk_raw = i32::from_be_bytes(weapon_perk_bytes);

    let weapon_perk = match weapon_perk_raw {
        -1 => Option::None,
        value => Option::Some(
            match object::common::critter::Perk::try_from(value) {
                Ok(value) => value,
                Err(_) =>
                    return Err(errors::Error::Format(errors::Format::Data))
            }
        ),
    };

    let mut weapon_burst_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_burst_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_burst_count = u32::from_be_bytes(weapon_burst_bytes);

    let mut weapon_caliber_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_caliber_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_caliber_raw = u32::from_be_bytes(weapon_caliber_bytes);

    let weapon_caliber =
        match weapon_caliber_raw {
            0 => None,
            value => Some(
                match object::common::weapons::Caliber::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let mut weapon_ammo_pid_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_ammo_pid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_ammo_pid = u32::from_be_bytes(weapon_ammo_pid_bytes);

    let mut weapon_capacity_bytes = [0u8; 4];
    match source.read_exact(&mut weapon_capacity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_capacity = u32::from_be_bytes(weapon_capacity_bytes);

    let mut weapon_sound_ids_bytes = [0u8; 1];
    match source.read_exact(&mut weapon_sound_ids_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let weapon_sound_ids = u8::from_be_bytes(weapon_sound_ids_bytes);

    Ok(object::item::weapon::Instance {
        flags: weapon_flags,
        damage: weapon_damage,
        attacks: [weapon_attack1, weapon_attack2],
        animation: weapon_animation,
        requirements: object::item::weapon::Requirements {
            strength: weapon_min_strength
        },
        rounds: object::item::weapon::Rounds {
            burst: weapon_burst_count,
            magazine: weapon_capacity,
        },
        caliber: weapon_caliber,
        perk: weapon_perk,
        connections: object::item::weapon::Connections {
            ammo_item_id: weapon_ammo_pid,
            failure_list_id: weapon_crit_list_idx,
            projectile_misc_id: weapon_projectile_idx,
            _sounds_ids: weapon_sound_ids,
        },
    })
}