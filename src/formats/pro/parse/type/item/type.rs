use std::collections::HashSet;

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
                                attack_modes: u8) -> Result<object::item::Type, errors::Error> {
    Ok(match type_id {
        0 => match armor::instance(source) {
            Ok(value) => object::item::Type::Armor(value),
            Err(error) => return Err(error)
        }
        1 => match container::instance(source) {
            Ok(value) => object::item::Type::Container(value),
            Err(error) => return Err(error)
        }
        2 => match drug::instance(source) {
            Ok(value) => object::item::Type::Drug(value),
            Err(error) => return Err(error)
        }
        3 => match weapon::instance(source, weapon_flags, attack_modes) {
            Ok(value) => object::item::Type::Weapon(value),
            Err(error) => return Err(error)
        }
        4 => match ammo::instance(source) {
            Ok(value) => object::item::Type::Ammo(value),
            Err(error) => return Err(error)
        }
        5 => match misc::instance(source) {
            Ok(value) => object::item::Type::Misc(value),
            Err(error) => return Err(error)
        }
        6 => match key::instance(source) {
            Ok(value) => object::item::Type::Key(value),
            Err(error) => return Err(error)
        }
        _ => return Err(errors::Error::Format),
    })
}