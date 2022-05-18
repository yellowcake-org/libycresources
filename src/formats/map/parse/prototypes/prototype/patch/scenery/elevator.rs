use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::formats::pro::object::scenery::elevator::Patch;
use crate::formats::pro::traits::TryFromOptional;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let r#type = u16::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    let elevation = source.read_i32::<BigEndian>()?;

    Ok(Patch { floor: elevation, r#type })
}