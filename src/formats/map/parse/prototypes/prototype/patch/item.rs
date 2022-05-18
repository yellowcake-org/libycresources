use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::item::{Body, Instance, Patch};

mod weapon;
mod ammo;
mod misc;
mod key;

pub(crate) fn patch<S: Read>(source: &mut S, item: &Instance) -> Result<Patch, errors::Error> {
    Ok(match item.r#type {
        Body::Armor(_) => { Patch::Armor(()) }
        Body::Container(_) => { Patch::Container(()) }
        Body::Drug(_) => { Patch::Drug(()) }
        Body::Weapon(_) => { Patch::Weapon(weapon::patch(source)?) }
        Body::Ammo(_) => { Patch::Ammo(ammo::patch(source)?) }
        Body::Misc(_) => { Patch::Misc(misc::patch(source)?) }
        Body::Key(_) => { Patch::Key(key::patch(source)?) }
    })
}