use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::formats::pro::object::common::map;
use crate::formats::pro::object::common::map::Map;
use crate::formats::pro::object::scenery::stairs;
use crate::formats::pro::object::scenery::stairs::Patch;
use crate::formats::pro::traits::TryFromOptional;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let mut destination_bytes = [0u8; 4];
    source.read_exact(&mut destination_bytes)?;

    Ok(Patch {
        destination: stairs::Destination {
            map: Map::try_from(source.read_i32::<BigEndian>()?)?,
            target: map::Destination::try_from_optional(&destination_bytes, &[0xFF, 0xFF, 0xFF, 0xFF])?,
        }
    })
}