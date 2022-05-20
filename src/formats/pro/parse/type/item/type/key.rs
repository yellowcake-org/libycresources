use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::key::Instance, errors::Error> {
    let mut code_bytes = [0u8; 4];
    match source.read_exact(&mut code_bytes) {
        Ok(value) => value,
        Err(error) => return Err(errors::Error::Read(error)),
    };

    let code = match u32::try_from_optional(i32::from_be_bytes(code_bytes), -1) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format)
    };

    Ok(object::item::key::Instance { code })
}