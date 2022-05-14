use crate::formats::map::common::Coordinate;

use super::super::*;

pub fn instance<S: Read>(source: &mut S) -> Result<state::object::Instance, errors::Error> {
    let mut id_bytes = [0u8; 4];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let id = u32::from_be_bytes(id_bytes);

    let mut position_bytes = [0u8; 4];
    match source.read_exact(&mut position_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let position = match Coordinate::try_from(u32::from_be_bytes(position_bytes)) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    todo!()
}