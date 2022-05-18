use crate::formats::pro::{ObjectInstance, ObjectPatch, ObjectType};

use super::super::*;

mod item;
mod critter;
mod scenery;
mod misc;

pub(crate) fn instance<S: Read, P: PrototypeProvider>
(source: &mut S, provider: &P, identifier: &Identifier<ObjectType>) -> Result<ObjectPatch, errors::Error> {
    let prototype = provider.provide(&identifier)?;

    Ok(match &prototype.object {
        ObjectInstance::Item(item) => { ObjectPatch::Item(item::patch(source, item)?) }
        ObjectInstance::Critter(_) => { ObjectPatch::Critter(critter::patch(source)?) }
        ObjectInstance::Scenery(scenery) => { ObjectPatch::Scenery(scenery::patch(source, scenery)?) }
        ObjectInstance::Wall(_) => { ObjectPatch::Wall(()) }
        ObjectInstance::Tile(_) => { ObjectPatch::Tile(()) }
        ObjectInstance::Misc(_) => { ObjectPatch::Misc(misc::patch(source, &identifier.value)?) }
    })
}