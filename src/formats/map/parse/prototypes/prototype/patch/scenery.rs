use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::{Instance, SceneryInstance, SceneryPatch};

mod door;
mod stairs;
mod elevator;
mod ladder;

pub(crate) fn patch<S: Read>(source: &mut S, scenery: &Instance) -> Result<SceneryPatch, errors::Error> {
    Ok(match &scenery.r#type {
        SceneryInstance::Door(door) => { SceneryPatch::Door(door::patch(source, door)?) }
        SceneryInstance::Stairs(stairs) => { SceneryPatch::Stairs(stairs::patch(source, stairs)?) }
        SceneryInstance::Elevator(elevator) => { SceneryPatch::Elevator(elevator::patch(source, elevator)?) }
        SceneryInstance::Ladder(ladder) => { SceneryPatch::Ladder(ladder::patch(source, ladder)?) }
        SceneryInstance::Generic(_) => { SceneryPatch::Generic(()) }
    })
}