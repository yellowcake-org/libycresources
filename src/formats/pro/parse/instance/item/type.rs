use std::collections::HashSet;
use object::item::Body;
use object::item::Type::*;

use super::super::super::*;

mod armor;
mod container;
mod drug;
mod weapon;
mod ammo;
mod misc;
mod key;

pub(crate) fn instance<S: Read>(source: &mut S, type_id: u32,
                                weapon_flags: HashSet<object::item::weapon::Flag>,
                                attack_modes: u8) -> Result<Body, errors::Error> {
    Ok(match type_id {
        0 => Armor(armor::instance(source)?),
        1 => Container(container::instance(source)?),
        2 => Drug(drug::instance(source)?),
        3 => Weapon(weapon::instance(source, weapon_flags, attack_modes)?),
        4 => Ammo(ammo::instance(source)?),
        5 => Misc(misc::instance(source)?),
        6 => Key(key::instance(source)?),
        _ => return Err(errors::Error::Format),
    })
}