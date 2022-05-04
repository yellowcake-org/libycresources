use libycresources::formats::pro;
use libycresources::formats::pro::object::Type;

mod item;

pub fn prototype(prototype: &pro::Prototype) {
    println!("Object ID: {:?}", prototype.id);
    println!("Text ID: {:?}", prototype.meta.connections.description_id);
    println!("Sprite ID: {:?}", prototype.meta.sprite.id);
    println!("Sprite type: {:?}", prototype.meta.sprite.r#type);
    println!("Light radius: {:?}", prototype.meta.light.distance);
    println!("Light intensity: {:?}", prototype.meta.light.intensity);
    println!("Flags: {:?}", prototype.meta.flags);

    println!();

    match &prototype.r#type {
        Type::Item(item) => { item::item(item) }
        Type::Critter(_) => {}
        Type::Scenery(_) => {}
        Type::Wall(_) => {}
        Type::Tile(_) => {}
        Type::Misc(_) => {}
    }
}