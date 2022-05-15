use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::geometry::Coordinate;
use crate::formats::map::common::{Elevation, Orientation};
use crate::formats::map::defaults::Instance;
use crate::formats::map::parse::errors;

pub fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    return Ok(Instance {
        position: Coordinate::try_from(source.read_u32::<BigEndian>()?)?,
        elevation: Elevation::try_from(source.read_u32::<BigEndian>()?)?,
        orientation: Orientation::try_from(source.read_u32::<BigEndian>()?)?,
    });
}