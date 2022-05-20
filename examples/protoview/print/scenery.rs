use libycresources::formats::pro::object::scenery::SceneryType;

mod door;
mod stairs;
mod elevator;
mod ladder;
mod generic;

pub(crate) fn scenery(scenery: &libycresources::formats::pro::object::scenery::Instance) {
    println!("Prototype is Scenery");
    println!();

    println!("Light: {:?}", scenery.light);
    println!("Script: {:?}", scenery.script);
    println!("Actions: {:?}", scenery.actions);
    println!("Material: {:?}", scenery.material);
    println!("Sound IDs: 0x{:x}", scenery.connections._sounds_ids);
    println!();

    match &scenery.body {
        SceneryType::Door(door) => { door::door(door) }
        SceneryType::Stairs(stairs) => { stairs::stairs(stairs) }
        SceneryType::Elevator(elevator) => { elevator::elevator(elevator) }
        SceneryType::Ladder(ladder) => { ladder::ladder(ladder) }
        SceneryType::Generic(generic) => { generic::generic(generic) }
    }
}