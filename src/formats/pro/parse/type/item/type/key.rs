use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::key::Instance, errors::Error> {
    let mut code_bytes = [0u8; 4];
    match source.read_exact(&mut code_bytes) {
        Ok(value) => value,
        Err(error) => return Err(errors::Error::Read(error)),
    };

    let code = u32::from_be_bytes(code_bytes);

    Ok(object::item::key::Instance { code })
}