use crate::formats::pro::object::scenery::ladder::Direction::*;

use super::super::*;

mod door;
mod stairs;
mod elevator;
mod ladder;
mod generic;

pub(crate) fn body<S: Read>(source: &mut S, type_id: u32) -> Result<object::scenery::Body, errors::Error> {
    Ok(match type_id {
        0 => object::scenery::Body::Door(match door::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        1 => object::scenery::Body::Stairs(match stairs::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        2 => object::scenery::Body::Elevator(match elevator::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        3 => object::scenery::Body::Ladder(match ladder::instance(source, Bottom) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        4 => object::scenery::Body::Ladder(match ladder::instance(source, Top) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        5 => object::scenery::Body::Generic(match generic::instance(source) {
            Ok(value) => value,
            Err(error) => return Err(error),
        }),
        _ => return Err(errors::Error::Format),
    })
}