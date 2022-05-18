use crate::formats::pro::object::item::key::Patch;

use super::super::super::*;

pub fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    Ok(Patch { code: u32::try_from(source.read_u32::<BigEndian>()?).ok() })
}