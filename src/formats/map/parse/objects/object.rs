use std::ops::Range;

use crate::common::types::Scaled;
use crate::formats::map::common::{Coordinate, Elevation, Orientation};

use super::super::*;
use super::super::super::super::super::*;

pub fn instance<S: Read>(source: &mut S) -> Result<state::object::Instance, errors::Error> {
    let entry_id = source.read_u32::<BigEndian>()?;
    let position = Coordinate::try_from(source.read_u32::<BigEndian>()?)?;

    let screen_shift_x = source.read_u32::<BigEndian>()?;
    let screen_shift_y = source.read_u32::<BigEndian>()?;

    let screen_position_x = source.read_i32::<BigEndian>()?;
    let screen_position_y = source.read_i32::<BigEndian>()?;

    let frame_idx = source.read_u32::<BigEndian>()?;
    let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

    let sprite_id = source.read_u32::<BigEndian>()?;

    let flags = source.read_u32::<BigEndian>()?;
    let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;

    let prototype_id = source.read_u32::<BigEndian>()?;
    let critter_idx = source.read_i32::<BigEndian>()?;

    let lradius = source.read_u32::<BigEndian>()?;
    let lintensity = source.read_u32::<BigEndian>()?;

    let outline_color = source.read_u32::<BigEndian>()?;
    let script_id = source.read_u32::<BigEndian>()?;

    let inventory_count = Scaled {
        value: source.read_u32::<BigEndian>()?,
        scale: 0..=source.read_u32::<BigEndian>()?,
    };

    Ok(state::object::Instance { reference_id: prototype_id })
}