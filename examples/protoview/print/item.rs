use libycresources::formats::pro::object::item::Type;

mod armor;
mod container;
mod ammo;
mod misc;

pub(crate) fn item(item: &libycresources::formats::pro::object::item::Instance) {
    println!("Prototype is Item");
    println!("Flags: {:?}", item.flags);
    println!("Actions: {:?}", item.actions);
    println!("Script: {:?}", item.script);
    println!("Material: {:?}", item.material);
    println!("Size: {:?}", item.size);
    println!("Weight: {:?}", item.weight);
    println!("Price: {:?}", item.price);

    match &item.sprite {
        None => { println!("No inventory sprite.") }
        Some(sprite) => {
            println!("Sprite ID: {:?}", sprite.id);
            println!("Sprite type: {:?}", sprite.r#type);
        }
    }

    println!("Sound IDs: 0x{:x}", item.connections._sounds_ids);
    println!();

    match &item.r#type {
        Type::Armor(armor) => { armor::armor(armor) }
        Type::Container(container) => { container::container(container) }
        Type::Drug(_) => {}
        Type::Weapon(_) => {}
        Type::Ammo(ammo) => { ammo::ammo(ammo) }
        Type::Misc(misc) => { misc::misc(misc) }
        Type::Key(_) => {}
    }
}