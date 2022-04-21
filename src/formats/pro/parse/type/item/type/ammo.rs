use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::ammo::Instance, errors::Error> {
    let mut ammo_caliber_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_caliber_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_caliber_raw = u32::from_be_bytes(ammo_caliber_bytes);

    let ammo_caliber =
        match ammo_caliber_raw {
            0 => None,
            value => Some(
                match object::common::weapons::Caliber::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let mut ammo_count_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_count = u32::from_be_bytes(ammo_count_bytes);

    let mut ammo_ac_modifier_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_ac_modifier_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_ac_modifier = u32::from_be_bytes(ammo_ac_modifier_bytes);

    let mut ammo_dr_modifier_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_dr_modifier_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_dr_modifier = u32::from_be_bytes(ammo_dr_modifier_bytes);

    let mut ammo_dmg_multiplier_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_dmg_multiplier_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_dmg_multiplier = u32::from_be_bytes(ammo_dmg_multiplier_bytes);

    let mut ammo_dmg_divider_bytes = [0u8; 4];
    match source.read_exact(&mut ammo_dmg_divider_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ammo_dmg_divider = u32::from_be_bytes(ammo_dmg_divider_bytes);

    Ok(object::item::ammo::Instance {
        count: ammo_count,
        caliber: ammo_caliber,
        adjustments: object::item::ammo::adjustments::Instance {
            armor: object::item::ammo::adjustments::Armor {
                class: ammo_ac_modifier,
                resistance: ammo_dr_modifier,
            },
            damage: object::item::ammo::adjustments::Damage {
                divider: ammo_dmg_divider,
                multiplier: ammo_dmg_multiplier,
            },
        },
    })
}