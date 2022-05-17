use crate::formats::pro::{ObjectInstance, ObjectPatch, ObjectType};

use super::super::*;

pub mod item;
pub mod misc;

pub(crate) fn instance<S: Read, P: PrototypeProvider>
(source: &mut S, provider: &P, identifier: &Identifier<ObjectType>) -> Result<ObjectPatch, errors::Error> {
    let prototype = provider.provide(&identifier)?;

    Ok(match prototype.r#type {
        ObjectInstance::Item(_) => { todo!() }
        ObjectInstance::Critter(_) => { todo!() }
        ObjectInstance::Scenery(_) => { todo!() }
        ObjectInstance::Wall(_) => { ObjectPatch::Wall(()) }
        ObjectInstance::Tile(_) => { ObjectPatch::Tile(()) }
        ObjectInstance::Misc(_) => { ObjectPatch::Misc(misc::patch(source, identifier)?) }
    })
}