use super::*;

mod item;
mod critter;

pub(crate) fn instance<S: Read>(source: &mut S, type_id: u8) -> Result<object::Type, errors::Error> {
    Ok(match type_id {
        0 => object::Type::Item(match item::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        1 => object::Type::Critter(match critter::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        // 2 => {}
        // 3 => {}
        // 4 => {}
        // 5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    })
}