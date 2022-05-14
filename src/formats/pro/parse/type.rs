use crate::common::types::models::prototype;

use super::*;

mod common;

mod item;
mod critter;
mod scenery;
mod wall;
mod tile;
mod misc;

pub(crate) fn instance<S: Read>(source: &mut S, kind: &prototype::Kind) -> Result<object::Type, errors::Error> {
    Ok(match kind {
        prototype::Kind::Item => object::Type::Item(item::instance(source)?),
        prototype::Kind::Critter => object::Type::Critter(critter::instance(source)?),
        prototype::Kind::Scenery => object::Type::Scenery(scenery::instance(source)?),
        prototype::Kind::Wall => object::Type::Wall(wall::instance(source)?),
        prototype::Kind::Tile => object::Type::Tile(tile::instance(source)?),
        prototype::Kind::Misc => object::Type::Misc(misc::instance(source)?),
    })
}