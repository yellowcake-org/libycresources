use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Elevation, Orientation};
use crate::common::types::models::Identifier;
use crate::formats::pro::object::common::map::{Destination, Map};
use crate::formats::pro::object::misc;
use crate::formats::pro::object::misc::exit::Instance;
use crate::formats::pro::ObjectType;

pub(crate) fn patch<S: Read>(source: &mut S, identifier: &Identifier<ObjectType>) ->
Result<Option<misc::Patch>, errors::Error> {
    Ok(if (0x0000..=0x0017).contains(&identifier.value) {
        let map = source.read_i32::<BigEndian>()?;
        let position = Coordinate::try_from(source.read_u32::<BigEndian>()?)?;
        let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;
        let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

        Some(
            misc::Patch::Exit(
                Instance {
                    map: Map::try_from(map)?,
                    destination: Destination { elevation, position },
                    orientation,
                }
            )
        )
    } else {
        None
    })
}