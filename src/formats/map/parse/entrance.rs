use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::geometry::{Coordinate, Elevation, Orientation};
use crate::formats::map::location;
use crate::formats::map::parse::errors;

pub fn instance<S: Read>(source: &mut S) -> Result<location::Grid, errors::Error> {
    return Ok(location::Grid {
        position: Coordinate::try_from(source.read_u32::<BigEndian>()?)?,
        elevation: Elevation::try_from(source.read_u32::<BigEndian>()?)?,
        orientation: Orientation::try_from(source.read_u32::<BigEndian>()?)?,
    });
}