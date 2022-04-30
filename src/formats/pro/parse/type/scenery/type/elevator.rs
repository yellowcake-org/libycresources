use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::elevator::Instance, errors::Error> {
    let mut type_bytes = [0u8; 4];
    match source.read_exact(&mut type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let r#type = u32::from_be_bytes(type_bytes);

    let mut floor_bytes = [0u8; 4];
    match source.read_exact(&mut floor_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let floor = u32::from_be_bytes(floor_bytes);
    
    Ok(object::scenery::elevator::Instance { floor, r#type })
}