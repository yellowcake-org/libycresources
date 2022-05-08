use std::io::Read;

use crate::common::types::ScaledValue;

use super::*;

pub fn instance<S: Read>(source: &mut S) -> Result<common::Defaults, errors::Error> {
    let mut default_position_bytes = [0u8; 4];
    match source.read_exact(&mut default_position_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_position = match u32::from_be_bytes(default_position_bytes) {
        value if (0..40000).contains(&value) => ScaledValue { value: value as u16, scale: 0u16..40000 },
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

    return Ok(common::Defaults {
        position: default_position,
        elevation: default_elevation,
        orientation: default_orientation,
    });
}