use crate::formats::pro::object::item::weapon::Patch;

use super::super::super::*;

pub fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let rounds = source.read_u32::<BigEndian>()?;
    let ammo_item_id = u16::try_from(source.read_i32::<BigEndian>()?).ok();

    Ok(Patch { rounds, ammo_item_id })
}