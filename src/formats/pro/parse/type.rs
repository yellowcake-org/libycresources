use super::*;

mod common;

mod item;
mod critter;
mod scenery;
mod wall;

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
        2 => object::Type::Scenery(match scenery::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        3 => object::Type::Wall(match wall::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        // 4 => {}
        // 5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    })
}