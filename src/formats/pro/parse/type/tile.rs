use super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::tile::Instance, errors::Error> {
    let mut material_id_bytes = [0u8; 4];
    match source.read_exact(&mut material_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let material = match object::common::world::Material::
    try_from(u32::from_be_bytes(material_id_bytes)) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format),
    };

    Ok(object::tile::Instance { material })
}