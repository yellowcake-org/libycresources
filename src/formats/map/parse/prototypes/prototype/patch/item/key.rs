use crate::common::traits::TryFromOptional;
use crate::formats::pro::object::item::key::Patch;

use super::super::super::*;

pub fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let code = i32::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;
    let code = code.map_or(None, |c| u32::try_from(c).ok());

    Ok(Patch { code })
}