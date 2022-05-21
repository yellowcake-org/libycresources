use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::traits::TryFromOptional;
use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Orientation};
use crate::common::types::space::Elevation;
use crate::formats::pro::object::common::map::{Destination, Map};
use crate::formats::pro::object::misc::exit::Instance;
use crate::formats::pro::object::misc::Patch;

pub(crate) fn patch<S: Read>(source: &mut S, id: &u16) ->
Result<Patch, errors::Error> {
    Ok(if (0x0010..=0x0017).contains(id) {
        let map = Map::try_from(source.read_i32::<BigEndian>()?)?;
        let position = Coordinate::try_from_optional(source.read_i32::<BigEndian>()?, -1)?;
        let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;
        let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

        position.map(|p| Patch::Exit(
            Instance {
                map,
                destination: Destination { elevation, position: p },
                orientation,
            }
        )).unwrap_or(Patch::None)
    } else {
        Patch::None
    })
}