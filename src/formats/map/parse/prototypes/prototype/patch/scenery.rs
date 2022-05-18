use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::{Body, Instance, Patch};

mod door;
mod stairs;
mod elevator;
mod ladder;

pub(crate) fn patch<S: Read>(source: &mut S, scenery: &Instance, read_ladders_map: bool) -> Result<Patch, errors::Error> {
    Ok(match &scenery.body {
        Body::Door(_) => { Patch::Door(door::patch(source)?) }
        Body::Stairs(_) => { Patch::Stairs(stairs::patch(source)?) }
        Body::Elevator(_) => { Patch::Elevator(elevator::patch(source)?) }
        Body::Ladder(_) => { Patch::Ladder(ladder::patch(source, read_ladders_map)?) }
        Body::Generic(_) => { Patch::Generic(()) }
    })
}