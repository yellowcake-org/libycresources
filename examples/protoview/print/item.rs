use ycresources::formats::pro::object::item::Type;

mod armor;
mod container;
mod drug;
mod weapon;
mod ammo;
mod misc;
mod key;

pub(crate) fn item(item: &ycresources::formats::pro::object::item::Instance) {
    println!("Prototype is Item");
    println!();
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
            println!("Sprite ID: {:?}", sprite.value);
            println!("Sprite type: {:?}", sprite.kind);
        }
    }

    println!("Sound IDs: {:#x}", item.connections._sounds_ids);
    println!();

    match &item.r#type {
        Type::Armor(armor) => { armor::armor(armor) }
        Type::Container(container) => { container::container(container) }
        Type::Drug(drug) => { drug::drug(drug) }
        Type::Weapon(weapon) => { weapon::weapon(weapon) }
        Type::Ammo(ammo) => { ammo::ammo(ammo) }
        Type::Misc(misc) => { misc::misc(misc) }
        Type::Key(key) => { key::key(key) }
    }
}