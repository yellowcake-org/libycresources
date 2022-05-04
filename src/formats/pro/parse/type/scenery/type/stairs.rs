use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::stairs::Instance, errors::Error> {
    let mut destination_bytes = [0u8; 4];
    match source.read_exact(&mut destination_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut map_bytes = [0u8; 4];
    match source.read_exact(&mut map_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let map_raw = i32::from_be_bytes(map_bytes);

    Ok(object::scenery::stairs::Instance {
        destination: object::scenery::stairs::Destination {
            map: match object::common::map::Map::try_from(map_raw) {
                Ok(value) => value,
                Err(error) => return Err(error),
            },
            target: match object::common::map::Destination::try_from(&destination_bytes) {
                Ok(value) => value,
                Err(error) => return Err(error),
            },
        }
    })
}