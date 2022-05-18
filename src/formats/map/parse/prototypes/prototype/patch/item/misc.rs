use crate::formats::pro::object::item::misc::Patch;

use super::super::super::*;

pub fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    Ok(Patch {
        count: u32::try_from(source.read_i32::<BigEndian>()?).ok()
    })
}