use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::{Instance, Body, Patch};

mod door;
mod stairs;
mod elevator;
mod ladder;

pub(crate) fn patch<S: Read>(source: &mut S, scenery: &Instance) -> Result<Patch, errors::Error> {
    Ok(match &scenery.body {
        Body::Door(_) => { Patch::Door(door::patch(source)?) }
        Body::Stairs(_) => { Patch::Stairs(stairs::patch(source)?) }
        Body::Elevator(elevator) => { Patch::Elevator(elevator::patch(source, elevator)?) }
        Body::Ladder(ladder) => { Patch::Ladder(ladder::patch(source, ladder)?) }
        Body::Generic(_) => { Patch::Generic(()) }
    })
}