use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::Scaled;
use crate::formats::map::common::{Coordinate, Elevation, Orientation};
use crate::formats::map::parse::errors;
use crate::formats::map::state;

pub fn instance<S: Read + Seek>(source: &mut S) -> Result<state::object::Instance, errors::Error> {
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

    let prototype_id = source.read_u32::<BigEndian>()?;
    let critter_idx = source.read_i32::<BigEndian>()?;

    let light_radius = source.read_u32::<BigEndian>()?;
    let light_intensity = source.read_u32::<BigEndian>()?;

    let outline_color = source.read_u32::<BigEndian>()?;
    let script_id = source.read_u32::<BigEndian>()?;

    let inventory_count = Scaled {
        value: source.read_u32::<BigEndian>()?,
        scale: 0..=source.read_u32::<BigEndian>()?,
    };

    source.seek(SeekFrom::Current(4))?;

    let flags_patch = source.read_u32::<BigEndian>()?;

    Ok(state::object::Instance { reference_id: prototype_id })
}