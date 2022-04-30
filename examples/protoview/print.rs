use libycresources::formats::pro;

pub fn prototype(prototype: &pro::Prototype) {
    println!("Common fields:");
    println!();
    println!("Object ID: {:?}", prototype.id);
    println!("Text ID: {:?}", prototype.meta.connections.description_id);
    println!("Sprite ID: {:?}", prototype.meta.sprite.id);
    println!("Sprite type: {:?}", prototype.meta.sprite.r#type);
    println!("Light radius: {:?}", prototype.meta.light.distance);
    println!("Light intensity: {:?}", prototype.meta.light.intensity);
    println!("Flags: {:?}", prototype.meta.flags);
}