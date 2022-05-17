use crate::formats::pro::object::item::misc::Patch;

use super::super::super::*;

pub fn instance<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    Ok(Patch { count: source.read_u32::<BigEndian>()? })
}