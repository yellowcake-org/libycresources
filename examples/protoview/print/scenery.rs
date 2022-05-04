use libycresources::formats::pro::object::scenery::Type;

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

    match &scenery.r#type {
        Type::Door(door) => { door::door(door) }
        Type::Stairs(stairs) => { stairs::stairs(stairs) }
        Type::Elevator(elevator) => { elevator::elevator(elevator) }
        Type::Ladder(ladder) => { ladder::ladder(ladder) }
        Type::Generic(generic) => { generic::generic(generic) }
    }
}