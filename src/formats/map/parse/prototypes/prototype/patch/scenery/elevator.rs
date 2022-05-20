use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::traits::TryFromOptional;
use crate::common::types::errors;
use crate::formats::pro::object::scenery::elevator::Patch;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let r#type = u16::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    let elevation = source.read_i32::<BigEndian>()?;

    Ok(Patch { floor: elevation, r#type })
}