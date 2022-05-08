use std::io::Read;

use crate::common::types::ScaledValue;
use crate::formats::map::defaults::{Instance, Position};
use crate::formats::map::parse::errors;

pub fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut default_position_bytes = [0u8; 4];
    match source.read_exact(&mut default_position_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    const SCALE: std::ops::Range<u8> = 0u8..200;
    let default_position = match u32::from_be_bytes(default_position_bytes) {
        value if (0..(SCALE.end as u32).pow(2)).contains(&value) => {
            let x = value / SCALE.end as u32;
            let y = value - (x * SCALE.end as u32);

            Position {
                x: ScaledValue { value: x as u8, scale: SCALE },
                y: ScaledValue { value: y as u8, scale: SCALE },
            }
        }
        _ => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut default_elevation_bytes = [0u8; 4];
    match source.read_exact(&mut default_elevation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_elevation = match u32::from_be_bytes(default_elevation_bytes) {
        value if (0..3).contains(&value) => ScaledValue { value: value as u8, scale: 0u8..3 },
        _ => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut default_orientation_bytes = [0u8; 4];
    match source.read_exact(&mut default_orientation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_orientation = match u32::from_be_bytes(default_orientation_bytes) {
        value if (0..6).contains(&value) => ScaledValue { value: value as u8, scale: 0u8..6 },
        _ => return Err(errors::Error::Format(errors::Format::Data))
    };

    return Ok(Instance {
        position: default_position,
        elevation: default_elevation,
        orientation: default_orientation,
    });
}