use super::*;

mod common;

mod item;
mod critter;
mod scenery;
mod wall;
mod tile;
mod misc;

pub(crate) fn instance<S: Read>(source: &mut S, r#type: &Type<(), (), (), (), (), ()>) ->
Result<ObjectInstance, errors::Error> {
    Ok(match r#type {
        Type::Item(_) => Type::Item(item::instance(source)?),
        Type::Critter(_) => Type::Critter(critter::instance(source)?),
        Type::Scenery(_) => Type::Scenery(scenery::instance(source)?),
        Type::Wall(_) => Type::Wall(wall::instance(source)?),
        Type::Tile(_) => Type::Tile(tile::instance(source)?),
        Type::Misc(_) => Type::Misc(misc::instance(source)?),
    })
}