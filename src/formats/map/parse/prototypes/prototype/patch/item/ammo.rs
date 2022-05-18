use crate::formats::pro::object::item::ammo::Patch;

use super::super::super::*;

pub fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    Ok(Patch { count: source.read_u32::<BigEndian>()? })
}