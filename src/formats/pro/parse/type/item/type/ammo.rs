use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::ammo::Instance, errors::Error> {
    let mut caliber_bytes = [0u8; 4];
    match source.read_exact(&mut caliber_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let caliber_raw = u32::from_be_bytes(caliber_bytes);

    let mut count_bytes = [0u8; 4];
    match source.read_exact(&mut count_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let count = u32::from_be_bytes(count_bytes);

    let mut ac_modifier_bytes = [0u8; 4];
    match source.read_exact(&mut ac_modifier_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let ac_modifier = i32::from_be_bytes(ac_modifier_bytes);

    let mut dr_modifier_bytes = [0u8; 4];
    match source.read_exact(&mut dr_modifier_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let dr_modifier = i32::from_be_bytes(dr_modifier_bytes);

    let mut dmg_multiplier_bytes = [0u8; 4];
    match source.read_exact(&mut dmg_multiplier_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let dmg_multiplier = u32::from_be_bytes(dmg_multiplier_bytes);

    let mut dmg_divider_bytes = [0u8; 4];
    match source.read_exact(&mut dmg_divider_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let dmg_divider = u32::from_be_bytes(dmg_divider_bytes);

    Ok(object::item::ammo::Instance {
        count,
        caliber: match object::common::weapons::Caliber::try_from_optional(caliber_raw, 0) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format)
        },
        adjustments: object::item::ammo::adjustments::Instance {
            armor: object::item::ammo::adjustments::Armor {
                class: ac_modifier,
                resistance: dr_modifier,
            },
            damage: object::item::ammo::adjustments::Damage {
                divider: dmg_divider,
                multiplier: dmg_multiplier,
            },
        },
    })
}