use crate::formats::pro::object::scenery::ladder::Direction::*;

use super::super::*;

mod door;
mod stairs;
mod elevator;
mod ladder;
mod generic;

pub(crate) fn body<S: Read>(source: &mut S, type_id: u32) -> Result<object::scenery::Body, errors::Error> {
    Ok(match type_id {
        0 => object::scenery::Body::Door(door::instance(source)?),
        1 => object::scenery::Body::Stairs(stairs::instance(source)?),
        2 => object::scenery::Body::Elevator(elevator::instance(source)?),
        3 => object::scenery::Body::Ladder(ladder::instance(source, Bottom)?),
        4 => object::scenery::Body::Ladder(ladder::instance(source, Top)?),
        5 => object::scenery::Body::Generic(generic::instance(source)?),
        _ => return Err(errors::Error::Format),
    })
}