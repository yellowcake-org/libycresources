use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::geometry::{Coordinate, Scaled};
use crate::common::types::models;
use crate::formats::map::blueprint;
use crate::formats::map::common::{Elevation, Orientation};
use crate::formats::map::parse::errors;

pub fn instance<S: Read + Seek>(source: &mut S) -> Result<blueprint::prototype::Instance, errors::Error> {
    let _entry_id = source.read_u32::<BigEndian>()?;

    let position = Coordinate::try_from(source.read_u32::<BigEndian>()?)?;

    let screen_shift = Coordinate {
        x: Scaled { value: source.read_u32::<BigEndian>()?, scale: u32::MIN..u32::MAX },
        y: Scaled { value: source.read_u32::<BigEndian>()?, scale: u32::MIN..u32::MAX },
    };

    let screen_position = Coordinate {
        x: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..i32::MAX },
        y: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..i32::MAX },
    };

    let frame_idx = source.read_u32::<BigEndian>()?;
    let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

    let sprite_id = source.read_u32::<BigEndian>()?;

    let flags = source.read_u32::<BigEndian>()?;
    let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;

    let identifier = models::Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let critter_idx = source.read_i32::<BigEndian>()?;

    let light_radius = source.read_u32::<BigEndian>()?;
    let light_intensity = source.read_u32::<BigEndian>()?;

    let outline_color = source.read_u32::<BigEndian>()?;
    let script_id = source.read_u32::<BigEndian>()?;

    let inventory_count = Scaled {
        value: source.read_u32::<BigEndian>()?,
        scale: u32::MIN..=source.read_u32::<BigEndian>()?,
    };

    source.seek(SeekFrom::Current(4))?;

    let flags_patch = source.read_u32::<BigEndian>()?;

    Ok(blueprint::prototype::Instance { identifier })
}