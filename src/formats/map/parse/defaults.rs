use std::io::Read;

use crate::formats::map::common::{Coordinate, Elevation, Orientation};
use crate::formats::map::defaults::Instance;
use crate::formats::map::parse::errors;

pub fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut position_bytes = [0u8; 4];
    match source.read_exact(&mut position_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let position =
        match Coordinate::try_from(u32::from_be_bytes(position_bytes)) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

    let mut elevation_bytes = [0u8; 4];
    match source.read_exact(&mut elevation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let elevation =
        match Elevation::try_from(u32::from_be_bytes(elevation_bytes)) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

    let mut orientation_bytes = [0u8; 4];
    match source.read_exact(&mut orientation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let orientation =
        match Orientation::try_from(u32::from_be_bytes(orientation_bytes)) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

    return Ok(Instance {
        position,
        elevation,
        orientation,
    });
}