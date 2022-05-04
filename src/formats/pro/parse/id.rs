use super::*;

pub(crate) fn instance(bytes: [u8; 4]) -> Result<(u8, u16), errors::Error> {
    let type_id = bytes[0];

    let object_id = u16::from_be_bytes(match &bytes[2..4].try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => *value,
    });

    Ok((type_id, object_id))
}