use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::elevator::Instance, errors::Error> {
    let mut type_bytes = [0u8; 4];
    match source.read_exact(&mut type_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let type_raw = i32::from_be_bytes(type_bytes);
    let r#type = match u16::try_from_optional(type_raw, -1) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format)
    };

    let mut floor_bytes = [0u8; 4];
    match source.read_exact(&mut floor_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let floor = i32::from_be_bytes(floor_bytes);

    Ok(object::scenery::elevator::Instance { floor, r#type })
}