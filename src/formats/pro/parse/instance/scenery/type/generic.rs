use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::scenery::generic::Instance, errors::Error> {
    let mut unknown_bytes = [0u8; 4];
    match source.read_exact(&mut unknown_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    Ok(object::scenery::generic::Instance { _unknown: u32::from_be_bytes(unknown_bytes) })
}