use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S, direction: object::scenery::ladder::Direction)
                                -> Result<object::scenery::ladder::Instance, errors::Error> {
    let mut destination_bytes = [0u8; 4];
    match source.read_exact(&mut destination_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    Ok(object::scenery::ladder::Instance {
        direction,
        destination: match object::common::map::Destination::
        try_from_optional(&destination_bytes, &[0xFF, 0xFF, 0xFF, 0xFF]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        },
    })
}