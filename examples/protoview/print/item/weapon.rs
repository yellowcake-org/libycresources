pub(crate) fn weapon(weapon: &libycresources::formats::pro::object::item::weapon::Instance) {
    println!("Item is Weapon");
    println!();

    println!("Flags: {:?}", weapon.flags);
    println!("Damage: {:?}", weapon.damage);
    println!("Primary attack: {:?}", &weapon.attacks[0]);
    println!("Secondary attack: {:?}", &weapon.attacks[1]);
    println!("Animation: {:?}", weapon.animation);
    println!("Requirements: {:?}", weapon.requirements);
    println!("Ammunition: {:?}", weapon.ammunition);
    println!("Perk: {:?}", weapon.perk);
    println!();

    println!("Connections");
    println!();
    println!("Ammo ID: {:?}", weapon.connections.ammo_item_id);
    println!("Failure ID: {:?}", weapon.connections.failure_list_id);
    println!("Projectile ID: {:?}", weapon.connections.projectile_misc_id);
    println!("Sound IDs: 0x{:x}", weapon.connections._sounds_ids);
}