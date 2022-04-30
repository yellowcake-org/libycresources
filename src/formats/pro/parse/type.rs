use super::*;

mod common;

mod item;
mod critter;
mod scenery;
mod wall;
mod tile;
mod misc;

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
        4 => object::Type::Tile(match tile::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        5 => object::Type::Misc(match misc::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    })
}